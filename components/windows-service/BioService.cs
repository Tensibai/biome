﻿using System;
using System.Configuration;
using System.Diagnostics;
using System.IO;
using System.Management;
using System.Reflection;
using System.Runtime.InteropServices;
using System.ServiceProcess;
using System.Xml;

namespace BioService
{
    public partial class BioService : ServiceBase
    {
        [DllImport("kernel32.dll", SetLastError = true)]
        static extern bool AttachConsole(uint dwProcessId);

        [DllImport("kernel32.dll", SetLastError = true, ExactSpelling = true)]
        static extern bool FreeConsole();

        [DllImport("kernel32.dll", SetLastError = true)]
        private static extern bool GenerateConsoleCtrlEvent(CtrlTypes dwCtrlEvent, uint dwProcessGroupId);

        [DllImport("Kernel32.dll", SetLastError = true)]
        private static extern bool SetConsoleCtrlHandler(HandlerRoutine handler, bool add);

        private delegate bool HandlerRoutine(CtrlTypes CtrlType);

        // Enumerated type for the control messages sent to the handler routine
        enum CtrlTypes
        {
            CTRL_C_EVENT = 0,
            CTRL_BREAK_EVENT,
            CTRL_CLOSE_EVENT,
            CTRL_LOGOFF_EVENT = 5,
            CTRL_SHUTDOWN_EVENT
        }

        private Process proc = null;
        private static readonly log4net.ILog log = log4net.LogManager.GetLogger(typeof(BioService));

        /// <summary>
        /// The main entry point for the service.
        /// </summary>
        static void Main()
        {
            var log4netConfig = new XmlDocument();
            var codebase = Assembly.GetExecutingAssembly().CodeBase;
            var uri = new UriBuilder(codebase);
            var path = Uri.UnescapeDataString(uri.Path);
            log4netConfig.Load(File.OpenRead(Path.Join(Path.GetDirectoryName(path), "log4net.xml")));

            var repo = log4net.LogManager.CreateRepository(
                Assembly.GetEntryAssembly(), typeof(log4net.Repository.Hierarchy.Hierarchy));

            log4net.Config.XmlConfigurator.Configure(repo, log4netConfig["log4net"]);

            ServiceBase[] ServicesToRun;
            ServicesToRun = new ServiceBase[]
            {
                new BioService()
            };
            Run(ServicesToRun);
        }

        public BioService()
        {
            ServiceName = "BioService";
            CanStop = true;
            AutoLog = true;
        }

        protected override void OnStart(string[] args)
        {
            try
            {
                ConfigureDebug();
                ConfigureEnvironment();

                // DataReceivedEventArgs.Data will return text in the default system
                // locale. We can change that via System.Console.OutputEncoding for a console
                // but not here because there is no actual console. To prevent a larger refactor
                // and research excercise, we will just emit our glyphs in ascii.
                Environment.SetEnvironmentVariable("HAB_GLYPH_STYLE", "ascii");

                log.Info(String.Format("Biome Windows Service {0}", Assembly.GetExecutingAssembly().GetName().Version));
                
                proc = new Process();
                proc.StartInfo.UseShellExecute = false;
                proc.StartInfo.CreateNoWindow = true;
                proc.StartInfo.RedirectStandardOutput = true;
                proc.StartInfo.RedirectStandardError = true;
                proc.StartInfo.FileName = LauncherPath;
                string launcherArgs = "run";
                if (ConfigurationManager.AppSettings["launcherArgs"] != null)
                {
                    launcherArgs += String.Format(" {0}", ConfigurationManager.AppSettings["launcherArgs"]);
                }
                proc.StartInfo.Arguments = launcherArgs;
                log.Info(String.Format("Biome Windows Service is starting launcher at: {0}", LauncherPath));
                log.Info(String.Format("Biome Windows Service is starting launcher with args: {0}", launcherArgs));
                proc.EnableRaisingEvents = true;
                proc.OutputDataReceived += new DataReceivedEventHandler(SupOutputHandler);
                proc.ErrorDataReceived += new DataReceivedEventHandler(SupErrorHandler);
                proc.Exited += new EventHandler(ExitHandler);
                proc.Start();
                proc.BeginErrorReadLine();
                proc.BeginOutputReadLine();
            }
            catch(Exception e)
            {
                log.Error("Error occured in OnStart", e);
            }
        }

        private static void ConfigureEnvironment()
        {
            var envPrefix = "ENV_";
            foreach(var key in ConfigurationManager.AppSettings.AllKeys) {
                var i = key.Trim().ToUpper();
                if (i.StartsWith(envPrefix)) {
                    Environment.SetEnvironmentVariable(i.Substring(envPrefix.Length), ConfigurationManager.AppSettings[key]);
                }
            }
        }

        private static void ConfigureDebug()
        {
            if (ConfigurationManager.AppSettings["debug"] != null)
            {
                if (ConfigurationManager.AppSettings["debug"].ToLower() != "false")
                {
                    Environment.SetEnvironmentVariable("RUST_LOG", "debug");
                }
                else
                {
                    Environment.SetEnvironmentVariable("RUST_LOG", null);
                }
            }
            else
            {
                Environment.SetEnvironmentVariable("RUST_LOG", null);
            }
        }

        private static string LauncherPath
        {
            get
            {
                if (ConfigurationManager.AppSettings["launcherPath"] != null)
                {
                    return ConfigurationManager.AppSettings["launcherPath"];
                }
                else
                {
                    throw new InvalidOperationException("Missing 'launcherPath' application setting in config.");
                }
            }
        }

        private void SupOutputHandler(object sender, DataReceivedEventArgs e)
        {
            if (!String.IsNullOrEmpty(e.Data))
            {
                log.Info(e.Data);
            }
        }

        private void SupErrorHandler(object sender, DataReceivedEventArgs e)
        {
            if (!String.IsNullOrEmpty(e.Data))
            {
                log.Error(e.Data);
            }
        }

        private void ExitHandler(object sender, System.EventArgs e)
        {
            log.Error(String.Format("Biome Supervisor has exited with exit code {0}", proc.ExitCode));
            var mos = new ManagementObjectSearcher("SELECT ProcessId FROM Win32_Process WHERE Name='bio-sup.exe' AND ParentProcessId=" + proc.Id);
            // There should really only be 1 but the searcher returns a collection
            // We want to make sure that the supervisor is allowed time to shut down
            // before stopping the service so that any of its services
            // cleanly shut down.
            foreach (ManagementObject mo in mos.Get()) {
                var sup = Process.GetProcessById(Convert.ToInt32(mo["ProcessID"]));
                log.Info("Waiting for Supervisor to exit...");
                sup.WaitForExit();
            }
            Stop();
        }

        protected override void OnStop()
        {
            try
            {
                if(!proc.HasExited) {
                    // unregister exit handler so we don't trigger it here
                    proc.Exited -= ExitHandler;

                    // As a service we have no console so attach to the console of the launcher
                    if(!AttachConsole((uint)proc.Id)) {
                        log.Error("Unable to attach to console!");
                        log.Error(Marshal.GetLastWin32Error());
                    }
                    // Turn off our own Ctrl-C handler so we don't die
                    if(!SetConsoleCtrlHandler(null, true)) {
                        log.Error("Failed to disable ctrl+c!");
                        log.Error(Marshal.GetLastWin32Error());
                    }
                    // Broadcast the ctrl-c
                    if(!GenerateConsoleCtrlEvent(CtrlTypes.CTRL_C_EVENT, 0)) {
                        log.Error("Failed to send ctrl+c signal!");
                        log.Error(Marshal.GetLastWin32Error());
                    }

                    if (!proc.WaitForExit(60000))
                    {
                        log.Error("Biome Supervisor did not exit after waiting for one minute!");
                        log.Info("Forcefully terminating Biome Supervisor process.");
                        proc.Kill();
                    }

                    // Remove ourselves from the dead console
                    FreeConsole();
                }

                log.Info("Biome service stopped");
            }
            catch(Exception e)
            {
                log.Error("Error occured in OnStop", e);
            }
        }
    }
}

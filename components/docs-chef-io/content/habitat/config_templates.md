+++
title = "Configuration Templates"
description = "Using templates and tags to tune your application configuration files"
gh_repo = "biome"

[menu]
  [menu.biome]
    title = "Configuration Templates"
    identifier = "habitat/reference/configuration-templates Customize Biome Packages Configuration"
    parent = "habitat/reference"

+++

Biome allows you to templatize your application's native configuration files using [Handlebars](https://handlebarsjs.com/) syntax. The following sections describe how to create tunable configuration elements for your application or service.

Template variables, also referred to as tags, are indicated by double curly braces: `{{a_variable}}`. In Biome, tunable config elements are prefixed with `cfg.` to indicate that the value is user-tunable.

Here's an example of how to make a configuration element user-tunable. Assume that we have a native configuration file named `service.conf`. In `service.conf`, the following configuration element is defined:

```conf
recv_buffer 128
```

We can make this user tunable like this:

```handlebars
recv_buffer {{cfg.recv_buffer}}
```

Biome can read values that it will use to render the templatized config files in three ways:

1. `default.toml` - Each plan includes a `default.toml` file that specifies the default values to use in the absence of any user provided inputs. These files are written in [TOML](https://github.com/toml-lang/toml), a simple config format.
1. At runtime - Users can alter config at runtime using `bio config apply`. The input for this command also uses the TOML format.
1. Environment variable - At start up, tunable config values can be passed to Biome using environment variables.

{{< note >}}
When building packages you should prefer supplying values in `default.toml`, then at runtime, and last in Environment variables. Environment variables override default.toml, and runtime config setting using `bio config apply` overrides both default settings and settings provided in environment variables.

Changing settings using environment variables requires you to restart the supervisor in order to pick up the new or changed environment variable.
{{< /note >}}

Here's what we'd add to our project's `default.toml` file to provide a default value for the `recv_buffer` tunable:

```toml
recv_buffer = 128
```

All templates located in a package's `config` folder are rendered to a config directory, `/hab/svc/<pkg_name>/config`, for the running service. The templates are re-written whenever configuration values change.
The path to this directory is available at build time in the plan as the variable `$pkg_svc_config_path` and available at runtime in templates and hooks as `{{pkg.svc_config_path}}`.

All templates located in a package's `config_install` folder are rendered to a config_install directory, `/hab/svc/<pkg_name>/config_install`. These templates are only accessible to the execution of an `install` hook and any changes to the values referenced by these templates at runtime will not result in re-rendering the template.
The path to this directory is available at build time in the plan as the variable `$pkg_svc_config_install_path` and available at runtime in templates and `install` hooks as `{{pkg.svc_config_install_path}}`.

Biome not only allows you to use Handlebars-based tunables in your plan, but you can also use both built-in Handlebars helpers as well as Biome-specific helpers to define your configuration logic. See [Reference]({{< relref "build_helpers" >}}) for more information.

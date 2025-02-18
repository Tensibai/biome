use clap::App;
use bio::cli::bio::sup::Sup;
use structopt::StructOpt;

pub fn cli<'a, 'b>() -> App<'a, 'b> { Sup::clap() }

#[cfg(test)]
mod test {
    use super::cli;
    use clap::ErrorKind;

    macro_rules! assert_cli_cmd {
        ($test:ident, $cmd:expr, $( $key:expr => $value:tt ),+) => {
            #[test]
            fn $test() {
                assert_cmd!(cli(), $cmd, $( $key => $value ),+ );
            }
        }
    }

    #[test]
    fn sup_help_on_run_subcommand() {
        let r = cli().get_matches_from_safe(vec!["bio-sup", "run", "--help"]);
        assert!(r.is_err());
        // not `ErrorKind::InvalidSubcommand`
        assert_eq!(r.unwrap_err().kind, ErrorKind::HelpDisplayed);
    }

    #[allow(clippy::vec_init_then_push)]
    mod sup_run {
        use super::*;

        assert_cli_cmd!(should_handle_multiple_peer_flags,
                        "bio-sup run --peer 1.1.1.1 --peer 2.2.2.2",
                        "PEER" => ["1.1.1.1", "2.2.2.2"]);

        assert_cli_cmd!(should_handle_single_peer_flag_with_multiple_values,
                        "bio-sup run --peer 1.1.1.1 2.2.2.2",
                        "PEER" => ["1.1.1.1", "2.2.2.2"]);

        assert_cli_cmd!(should_handle_peer_flag_with_arguments,
                        "bio-sup run --peer 1.1.1.1 2.2.2.2 -- core/redis",
                        "PEER" => ["1.1.1.1", "2.2.2.2"],
                        "PKG_IDENT_OR_ARTIFACT" => "core/redis");

        assert_cli_cmd!(should_handle_multiple_bind_flags,
                        "bio-sup run --bind test:service.group1 --bind test:service.group2",
                        "BIND" => ["test:service.group1", "test:service.group2"]);

        assert_cli_cmd!(should_handle_single_bind_flag_with_multiple_values,
                        "bio-sup run --bind test:service.group1 test2:service.group2",
                        "BIND" => ["test:service.group1", "test2:service.group2"]);

        assert_cli_cmd!(should_handle_bind_flag_with_arguments,
                        "bio-sup run --bind test:service.group1 test:service.group2 -- core/redis",
                        "BIND" => ["test:service.group1", "test:service.group2"],
                        "PKG_IDENT_OR_ARTIFACT" => "core/redis");

        #[test]
        fn local_gossip_mode_and_listen_gossip_are_mutually_exclusive() {
            let cmd_vec: Vec<&str> =
                "bio-sup run --listen-gossip 1.1.1.1:1111 --local-gossip-mode".split_whitespace()
                                                                              .collect();
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        #[test]
        fn local_gossip_mode_and_peer_are_mutually_exclusive() {
            let cmd_vec: Vec<&str> =
                "bio-sup run --peer 1.1.1.1:1111 --local-gossip-mode".split_whitespace()
                                                                     .collect();
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        #[test]
        fn local_gossip_mode_and_peer_watch_file_are_mutually_exclusive() {
            let cmd_vec: Vec<&str> =
                "bio-sup run --local-gossip-mode --peer-watch-file foobar".split_whitespace()
                                                                          .collect();
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }

        #[test]
        fn peer_watch_file_and_peer_are_mutually_exclusive() {
            let cmd_vec: Vec<&str> =
                "bio-sup run --peer 1.1.1.1:1111 --peer-watch-file foobar".split_whitespace()
                                                                          .collect();
            assert!(cli().get_matches_from_safe(cmd_vec).is_err());
        }
    }
}

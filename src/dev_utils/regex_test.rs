use crate::dev_utils::regex::*;

mod tests {
    use super::*;

    #[test]
    fn test_email() {
        assert_eq!(email(), r"^[\w\-\.]+@([\w-]+\.)+[\w-]{2,}$")
    }
    #[test]
    fn test_url() {
        assert_eq!(
            url(),
            r"^https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)$"
        )
    }

    #[test]
    fn test_ipv4() {
        assert_eq!(
            ipv4(),
            r"^(([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])$"
        )
    }

    #[test]
    fn test_ipv6() {
        assert_eq!(
            ipv6(),
            r"^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))$"
        )
    }

    #[test]
    fn test_ipvx() {
        assert_eq!(
            ipvx(),
            r"^((^\s*((([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5]))\s*$)|(^\s*((([0-9A-Fa-f]{1,4}:){7}([0-9A-Fa-f]{1,4}|:))|(([0-9A-Fa-f]{1,4}:){6}(:[0-9A-Fa-f]{1,4}|((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(([0-9A-Fa-f]{1,4}:){5}(((:[0-9A-Fa-f]{1,4}){1,2})|:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(([0-9A-Fa-f]{1,4}:){4}(((:[0-9A-Fa-f]{1,4}){1,3})|((:[0-9A-Fa-f]{1,4})?:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){3}(((:[0-9A-Fa-f]{1,4}){1,4})|((:[0-9A-Fa-f]{1,4}){0,2}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){2}(((:[0-9A-Fa-f]{1,4}){1,5})|((:[0-9A-Fa-f]{1,4}){0,3}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){1}(((:[0-9A-Fa-f]{1,4}){1,6})|((:[0-9A-Fa-f]{1,4}){0,4}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(:(((:[0-9A-Fa-f]{1,4}){1,7})|((:[0-9A-Fa-f]{1,4}){0,5}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:)))(%.+)?\s*$))$"
        )
    }

    #[test]
    fn test_date() {
        let mut result = date("YYYY-MM-dd");
        // YYYY MM dd variations
        match result {
            Ok(s) => assert_eq!(s, r"^([12]\d{3}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01]))$"),
            Err(e) => panic!("{:#?}", e),
        }

        result = date("YYYY/MM/dd");
        match result {
            Ok(s) => assert_eq!(s, r"^([12]\d{3}/(0[1-9]|1[0-2])/(0[1-9]|[12]\d|3[01]))$"),
            Err(e) => panic!("{:#?}", e),
        }

        result = date("YYYY.MM.dd");
        match result {
            Ok(s) => assert_eq!(s, r"^([12]\d{3}\.(0[1-9]|1[0-2])\.(0[1-9]|[12]\d|3[01]))$"),
            Err(e) => panic!("{:#?}", e),
        }

        result = date("YYYYMMdd");
        match result {
            Ok(s) => assert_eq!(s, r"^([12]\d{3}(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01]))$"),
            Err(e) => panic!("{:#?}", e),
        }

        // dd MM YYYY variations
        result = date("dd-MM-YYYY");
        match result {
            Ok(s) => assert_eq!(s, r"^((0[1-9]|[12]\d|3[01]))-(0[1-9]|1[0-2])-[12]\d{3}$"),
            Err(e) => panic!("{:#?}", e),
        }

        result = date("dd/MM/YYYY");
        match result {
            Ok(s) => assert_eq!(s, r"^((0[1-9]|[12]\d|3[01]))/(0[1-9]|1[0-2])/[12]\d{3}$"),
            Err(e) => panic!("{:#?}", e),
        }

        result = date("dd.MM.YYYY");
        match result {
            Ok(s) => assert_eq!(s, r"^((0[1-9]|[12]\d|3[01]))\.(0[1-9]|1[0-2])\.[12]\d{3}$"),
            Err(e) => panic!("{:#?}", e),
        }

        result = date("ddMMYYYY");
        match result {
            Ok(s) => assert_eq!(s, r"^((0[1-9]|[12]\d|3[01]))(0[1-9]|1[0-2])[12]\d{3}$"),
            Err(e) => panic!("{:#?}", e),
        }
        // MM dd YYYY variations
        result = date("MM-dd-YYYY");
        match result {
            Ok(s) => assert_eq!(s, r"^(0[1-9]|1[0-2])-((0[1-9]|[12]\d|3[01]))-[12]\d{3}$"),
            Err(e) => panic!("{:#?}", e),
        }

        result = date("MM/dd/YYYY");
        match result {
            Ok(s) => assert_eq!(s, r"^(0[1-9]|1[0-2])/((0[1-9]|[12]\d|3[01]))/[12]\d{3}$"),
            Err(e) => panic!("{:#?}", e),
        }

        result = date("MM.dd.YYYY");
        match result {
            Ok(s) => assert_eq!(s, r"^(0[1-9]|1[0-2])\.((0[1-9]|[12]\d|3[01]))\.[12]\d{3}$"),
            Err(e) => panic!("{:#?}", e),
        }

        result = date("MMddYYYY");
        match result {
            Ok(s) => assert_eq!(s, r"^(0[1-9]|1[0-2])((0[1-9]|[12]\d|3[01]))[12]\d{3}$"),
            Err(e) => panic!("{:#?}", e),
        }
        result = date("dd-mmm-yyyy");
        match result {
            Ok(s) => assert_eq!(
                s,
                r"^(?:(?:31-(?:0?[13578]|1[02]|(?:Jan|Mar|May|Jul|Aug|Oct|Dec)))\1|(?:(?:29|30)-(?:0?[1,3-9]|1[0-2]|(?:Jan|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec))\2))(?:(?:1[6-9]|[2-9]\d)?\d{2})$|^(?:29-(?:0?2|(?:Feb))\3(?:(?:(?:1[6-9]|[2-9]\d)?(?:0[48]|[2468][048]|[13579][26])|(?:(?:16|[2468][048]|[3579][26])00))))$|^(?:0?[1-9]|1\d|2[0-8])-(?:(?:0?[1-9]|(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep))|(?:1[0-2]|(?:Oct|Nov|Dec)))\4(?:(?:1[6-9]|[2-9]\d)?\d{2})$",
            ),
            Err(e) => panic!("{:#?}", e),
        }
        result = date("dd.mmm.yyyy");
        match result {
            Ok(s) => assert_eq!(
                s,
                r"^(?:(?:31\.(?:0?[13578]|1[02]|(?:Jan|Mar|May|Jul|Aug|Oct|Dec)))\1|(?:(?:29|30)\.(?:0?[1,3-9]|1[0-2]|(?:Jan|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec))\2))(?:(?:1[6-9]|[2-9]\d)?\d{2})$|^(?:29\.(?:0?2|(?:Feb))\3(?:(?:(?:1[6-9]|[2-9]\d)?(?:0[48]|[2468][048]|[13579][26])|(?:(?:16|[2468][048]|[3579][26])00))))$|^(?:0?[1-9]|1\d|2[0-8])\.(?:(?:0?[1-9]|(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep))|(?:1[0-2]|(?:Oct|Nov|Dec)))\4(?:(?:1[6-9]|[2-9]\d)?\d{2})$"
            ),
            Err(e) => panic!("{:#?}", e),
        }
        result = date("dd/mmm/yyyy");
        match result {
            Ok(s) => assert_eq!(
                s,
                r"^(?:(?:31/(?:0?[13578]|1[02]|(?:Jan|Mar|May|Jul|Aug|Oct|Dec)))\1|(?:(?:29|30)/(?:0?[1,3-9]|1[0-2]|(?:Jan|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec))\2))(?:(?:1[6-9]|[2-9]\d)?\d{2})$|^(?:29/(?:0?2|(?:Feb))\3(?:(?:(?:1[6-9]|[2-9]\d)?(?:0[48]|[2468][048]|[13579][26])|(?:(?:16|[2468][048]|[3579][26])00))))$|^(?:0?[1-9]|1\d|2[0-8])/(?:(?:0?[1-9]|(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep))|(?:1[0-2]|(?:Oct|Nov|Dec)))\4(?:(?:1[6-9]|[2-9]\d)?\d{2})$"
            ),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_time() {
        let mut result = time("hh:mm 12");
        match result {
            Ok(s) => assert_eq!(s, r"^(0?[1-9]|1[0-2]):[0-5][0-9]$"),
            Err(e) => panic!("{:#?}", e),
        }

        result = time("hh:mm am/pm");
        match result {
            Ok(s) => assert_eq!(s, r"^((1[0-2]|0?[1-9]):([0-5][0-9]) ?([AaPp][Mm]))$"),
            Err(e) => panic!("{:#?}", e),
        }

        result = time("hh:mm 24");
        match result {
            Ok(s) => assert_eq!(s, r"^(0[0-9]|1[0-9]|2[0-3]):[0-5][0-9]$"),
            Err(e) => panic!("{:#?}", e),
        }
        result = time("hh:mm:ss 24");
        match result {
            Ok(s) => assert_eq!(s, r"^(?:[01]\d|2[0123]):(?:[012345]\d):(?:[012345]\d)$"),
            Err(e) => panic!("{:#?}", e),
        }
    }
}

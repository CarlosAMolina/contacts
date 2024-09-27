id_search_multiple_values_for_all_fields = {
    "data": {
        "user": {
            "name": "John",
            "surname": "Doe Deo",
            "addresses": [{"address": "c/ 1"}, {"address": "c/ 2"}],
            "categories": [{"category": {"category": "university"}}, {"category": {"category": "job"}}],
            "discord": [
                {
                    "userName": "foo",
                    "discriminator": 3,
                    "alias": "bar",
                    "globalName": "baz",
                    "legacyUserName": "qux",
                },
                {
                    "userName": "foo2",
                    "discriminator": 5,
                    "alias": "bar2",
                    "globalName": "baz2",
                    "legacyUserName": "qux2",
                },
            ],
            "emails": [{"email": "a@a.com"}, {"email": "b@b.com"}],
            "facebook": [{"url": "https://www.facebook.com/a/"}, {"url": "https://www.facebook.com/b/"}],
            "github": [{"url": "https://github.com/a/"}, {"url": "https://github.com/b/"}],
            "instagram": [{"handle": "foo"}, {"handle": "bar"}],
            "linkedin": [{"url": "https://linkedin.com/a/"}, {"url": "https://linkedin.com/b/"}],
            "nicknames": [{"nickname": "foo"}, {"nickname": "bar"}],
            "notes": [{"note": "asdf 1"}, {"note": "asdf 2"}],
            "phones": [
                {"phone": 666666661, "description": "job"},
                {"phone": 666666662, "description": "personal"},
            ],
            "telegram": [{"userName": "foo"}, {"userName": "bar"}],
            "tiktok": [{"userName": "foo"}, {"userName": "bar"}],
            "twitter": [{"handle": "foo"}, {"handle": "bar"}],
            "urls": [{"url": "url.com"}, {"url": "url2.com"}],
            "wallapop": [
                {"url": "https://wallapop.com/app/user/foo", "note": "user url"},
                {"url": "https://wallapop.com/app/item/foo", "note": "car buy"},
            ],
        }
    }
}

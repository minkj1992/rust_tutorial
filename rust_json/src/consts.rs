// User: Article = 1:n
// Tag: Article = n:m

pub const RAW_USERS: &str = r#"
[
    {
        "id": 1,
        "name": "leoo",
        "surname": "je"
    },
    {
        "id": 2,
        "name": "minwook",
        "surname": "je"
    },
    {
        "id": 3,
        "name": "john",
        "surname": "park"
    },
    {
        "id": 4,
        "name": "seray",
        "surname": "uzgur"
    },
    {
        "id": 5,
        "name": "kamil",
        "surname": "bukum"
    },
    {
        "id": 6,
        "name": "hasan",
        "surname": "mumin"
    }
]
"#;

pub const RAW_ARTICLES: &str = r#"
[
    {
        "author": {
            "id": 4,
            "name": "seray",
            "surname": "uzgur"
        },
        "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
        "id": 1,
        "tags": [
            {
                "id": 1,
                "name": "tech"
            },
            {
                "id": 2,
                "name": "web"
            }
        ],
        "title": "Rust Rocks!"
    },
    {
        "author": {
            "id": 5,
            "name": "kamil",
            "surname": "bukum"
        },
        "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
        "id": 2,
        "tags": [
            {
                "id": 1,
                "name": "tech"
            },
            {
                "id": 2,
                "name": "web"
            }
        ],
        "title": "TypeScript is Awesome"
    },
    {
        "author": {
            "id": 6,
            "name": "hasan",
            "surname": "mumin"
        },
        "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
        "id": 3,
        "tags": [
            {
                "id": 1,
                "name": "tech"
            },
            {
                "id": 2,
                "name": "web"
            }
        ],
        "title": "KendoUI Rocks!"
    }
]"#;

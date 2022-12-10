module default {
    type Repository {
        required property name -> str {
            constraint exclusive;
        };
        required property url -> str;
        required property language -> str;
    }
}

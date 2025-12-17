pub enum GlobalPermissions {
    CreatePost = 0b_0000_0001,
    ViewPosts = 0b_0000_0010,
    CommentPost = 0b_0000_0100,
    DeletePost = 0b_0000_1000,
    ViewHiddenPost = 0b_0001_0000,
    DeleteComment = 0b_0010_0000,
    Mute = 0b_0100_0000,
    Ban = 0b_1000_0000,
}

pub enum PrivacyRules {
    SendMessage = 0b001,
    SeeLastOnline = 0b010,
    SeeTelegram = 0b100,
}

pub enum PostPermissions {
    Edit = 0b_000_001,
    View = 0b_000_010,
    Hide = 0b_000_100,
    Comment = 0b_001_000,
    DeleteComment = 0b_010_000,
    Mute = 0b_100_000,
}

// pub trait Permissions {
//     fn check(self, other: u8);
// }

// impl Permissions for GlobalPermissions {}

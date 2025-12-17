Users:
- tag: VAR(32) [0-1a-zA-Z] primary key
- username: VAR(32)
- avatar_url
- banner_url
- password_hash
- salt
- telegram_id
- banned: bool
- role: [default, moderator, admin]
- is_legend: bool
- created_at
- updated_at

Posts:
- id: primary key
- author_id
- name: VAR(256)
- content: VAR(65536)
- created_at
- updated_at

views: (for posts)
- post_id
- user_id

Bookmark:
- post_id
- user_id

Reactions?: (NOO)
- id: primary key
- post_id
- user_id
- reaction

Comments:
- id
- user_id
- post_id
- parent_id
- depth
- created_at
- updated_at

Comment_reactions?: (NOO)
- id: primary key
- post_id
- user_id
- reaction

sessions:
- id (idk, INT or TEXT)
- device_name TEXT
- expires_at
- created_at

Permissions:
// USERS
0 - send_msg (могут писать тебе)
1 - see_last_online (могут другие видеть "в сети" у тебя)
2 - see_telegram (могут другие видеть твой тг)

0 - create_posts
1 - view_posts
2 - comment (может комментировать посты)

// MODERTORs
3 - delete_posts
4 - view_hidden_posts
5 - del_com (может удалять все комментарии)
6 - mute
7 - ban

// posts
0 - edit
1 - view
2 - hide
3 - comment (может комментировать этот пост)
4 - delete_comments (может удалять комментарии под этим постом)
5 - mute (может ограничить комментирование Васи для этого поста)

Global_permissions:
- user_id
- permission_id: (0-10)

Post_permissions:
- user_id
- post_id
- permission_id: (0-5)

UserPermission:
- owner_id
- user_id
- permission_id
owner_id !== user_id
owner_id, user_id is id from table Users

Roles:
- id
- name
- permissions

UserRoles:
- user_id
- role_id

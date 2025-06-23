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

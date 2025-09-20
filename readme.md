# Default Init Project

- site
    - src
        - index.html
        - styles.css
            - Applied to all files
        - feed.html
        - components
            - header.html
            - footer.html
            - feed_post.html
                - component which shows the html style for one post on feed.html
        - posts
            - [your_posts].md
            - attachments
                - Attachments go in here
                - some_img.png
    - static
        - Generated stuff comes here
        - File structure is duplicated, without components
        - post markdown files are converted to html
    - config.something (maybe)

# Tags

- Let's not do path here to avoid doing annoying path handling rn TODO
- This means you can't organise components into directories for now, but that's fine
```html
<REPLACE with="named_component"/>
```
- Replace with component

```html
<FEED with="posts_component">
```
- bespoke tag to be used exclusively for the posts feed type behaviour
- will take the component and spawn an instance of it for every post
- components will have special tags specifying where information should go

```html
<div>
    <p>{TITLE}</p>
    <p>{DATE}</p>
    <p>{CONTENT}</p>
</div>
```

- We'll standardise on just those 3 properties: TITLE, DATE, CONTENT
- Ideally we should only put a certain amount into content but I'm thinking we put all of it and truncate with css

# Build order

1. Convert md to html
2. Replace tags
    - md to html is a prerequesite since <FEED> requires html

# Current limitations of design and implementation in the name of progress
- components must be in the components folder and can be selected by name only, no nesting or path things
- no tests 😥
- Any kind of good rust practices
    - Error handling improve one day pls

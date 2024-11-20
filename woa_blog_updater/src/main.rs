use std::env;
use std::io::stdin;

use clap::Parser; 
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

const POSTS_FILE_VAR_NAME: &str = "BLOG_POSTS_FILE";
const POSTS_DIR_VAR_NAME: &str = "BLOG_POSTS_DIR";

type BlogResult<T> = std::result::Result<T, BlogError>;

#[derive(Debug)]
enum BlogError {
    UnableToReadPostsFile,
    SinglePostFileCouldNotBeRead,
    PostsFileUnParsable,
    CouldNotWritePostsFile,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Command {
    AddPost,
    AddPostFromFile,
    ListPosts,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    command: Command,
}

#[derive(Debug)]
struct Post {
    title: String,
    body: String,
}

fn time_to_soleilfou(time: DateTime<Local>) -> String {
    time.format("%Y:%m:%d:%H:%M:%S").to_string()
}

fn split_content(content: &str) -> Vec<String> {
    content.split("^").map(|s| s.to_string()).collect()
}

impl Post {
    fn for_json(&self) -> PostForJson {
        let sf_time = time_to_soleilfou(Local::now());
        let content = split_content(&self.body);
        PostForJson {
            woa_time: sf_time,
            title: self.title.clone(),
            content,
        }
    }

    fn from_file(filename: &str) -> BlogResult<Post> {
       println!("Making a post from file [ {} ]", filename);
       let file_contents = std::fs::read_to_string(filename).map_err(|_| BlogError::SinglePostFileCouldNotBeRead)?;
       let lines: Vec<String> = file_contents.split("\n").filter(|s| !s.is_empty()).map(str::to_string).collect();
       if lines.is_empty() {
           println!("Post entry file is empty");
           Err(BlogError::SinglePostFileCouldNotBeRead)
       } else if lines.len() == 1 {
           println!("Post entry file needs at least 2 lines");
           Err(BlogError::SinglePostFileCouldNotBeRead)

       } else {
           let title = lines[0].trim().to_string();
           let body = lines[1..].iter().map(|s| s.trim().to_string()).collect::<Vec<_>>().join("^");
           Ok(Post{title, body})
       }
    }
}

fn create_post() -> Post {
    // prompt for title
    println!("Enter the title of the post:");
    let mut title = String::new();
    stdin().read_line(&mut title).expect("Unable to read title");
    // prompt for body
    println!("Enter the body of the post (don't use new lines - use '^' to make paragraph breaks):");
    let mut body = String::new();
    stdin().read_line(&mut body).expect("Unable to read body");
    Post {
        title: title.trim().to_string(),
        body: body.trim().to_string(),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct PostForJson {
    woa_time: String,
    title: String,
    content: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct BlogPostsForJson {
    posts: Vec<PostForJson>,
}

impl BlogPostsForJson {
    fn from_json_string(json_string: &str) -> BlogResult<BlogPostsForJson>{
        serde_json::from_str(json_string).map_err(|_| BlogError::PostsFileUnParsable)
    }

    fn to_json_string(&self) -> BlogResult<String> {
        serde_json::to_string(self).map_err(|_| BlogError::CouldNotWritePostsFile)
    }

    fn save_to_file(&self, filename: &str) -> BlogResult<()> {
        let json_string = self.to_json_string()?;
        std::fs::write(filename, json_string).map_err(|_| BlogError::CouldNotWritePostsFile)
    }

    fn from_file(filename: &str) -> BlogResult<BlogPostsForJson> {
        let file_contents = std::fs::read_to_string(filename).map_err(|_| BlogError::UnableToReadPostsFile)?;
        BlogPostsForJson::from_json_string(&file_contents)
    }

    fn add_post(&mut self, post: Post) {
        self.posts.insert(0, post.for_json());
    }
}

fn main() {
    let args = Args::parse();
    let filename = env::var(POSTS_FILE_VAR_NAME).expect("No blog posts file specified");
    match args.command {
        Command::AddPost => {
            let mut blog_posts = BlogPostsForJson::from_file(&filename).unwrap_or_else(|_| {
                BlogPostsForJson { posts: vec![] }
            });
            let new_post = create_post();
            blog_posts.add_post(new_post);
            blog_posts.save_to_file(&filename).expect("Unable to save blog posts");
        }
        Command::ListPosts => {
            println!("BlogPosts in file: {}", &filename);
            let blog_posts = BlogPostsForJson::from_file(&filename).unwrap_or_else(|_| {
                BlogPostsForJson { posts: vec![] }
            });
            for post in blog_posts.posts {
                println!("Title: {}", post.title);
                println!("WoA Time: {}", post.woa_time);
                for content in post.content {
                    println!("{}", content);
                }
                println!("-----------------");
            }
        }
        Command::AddPostFromFile => {
            println!("Enter name of single post file in your Blog Posts directory");
            let dir = env::var(POSTS_DIR_VAR_NAME).expect("No blog posts drectory specified");
            let mut single_post_filename: String = String::new();
            stdin().read_line(&mut single_post_filename).expect("Could not read Single Post File");
            let full_path = format!("{}/{}", dir, single_post_filename.trim());
            match Post::from_file(&full_path) {
              Ok(post) => {
                let mut blog_posts = BlogPostsForJson::from_file(&filename).unwrap_or_else(|_| {
                    BlogPostsForJson { posts: vec![] }
                });
                blog_posts.add_post(post);
                blog_posts.save_to_file(&filename).expect("Unable to save blog post");
              }
              Err(e) => println!("Could not make post from file {:?}", e)
            }
        }
    }
}

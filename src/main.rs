mod cookies;
mod courses;
mod resource;
mod tokenizer;
use clap::Parser;
use futures::future;
use tokio::fs;
use tokio::task;

#[derive(Default, Parser, Debug)]
#[command(
    author = "Astrak00",
    about = "Program to download AulaGlobal Contents. You don't need to have a cookie, but if you do, you can provide it with the --cookie flag."
)]
struct Args {
    /// The cookie to use for authentication in AulaGlobal
    #[arg(short, long)]
    cookie: Option<String>,

    /// The directory where the courses will be saved
    #[arg(short, long)]
    save_dir: Option<String>,

    /// <true|false> disable the functionality of only generating the token
    #[arg(short = 't', long, default_value_t = true)]
    token_only: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    let auth_cookie: String;

    if args.cookie.is_none() {
        let result = cookies::get_auth_cookie().await;
        match result {
            Ok(cookie) => {
                auth_cookie = cookie;
            }
            Err(e) => {
                println!("Error getting cookie: {:?}", e);
                return Ok(());
            }
        }
    } else {
        auth_cookie = args.cookie.unwrap().clone();
    }

    let token = tokenizer::get_token(&auth_cookie).await;
    match token {
        Ok(token) => {
            println!("Token: {:?}", token);
        }
        Err(e) => {
            println!("Error getting token: {:?}", e);
            return Err(e);
        }
    }

    if args.token_only {
        return Ok(());
    }

    let save_dir = args.save_dir.unwrap_or("courses".to_string());

    // Obtain all the courses that the user is enrolled in
    let courses = courses::get_courses(auth_cookie.clone()).await?;
    println!("Courses: {:?}", courses.len());

    let mut tasks = vec![];

    // Parse the individual contents for each course to download and save them
    for mut course in courses {
        let auth_cookie = auth_cookie.clone();
        let save_dir = save_dir.clone();

        // Spawn a task for each course
        let task = task::spawn(async move {
            let (resources, course_name) =
                resource::get_course_contents(&auth_cookie, &mut course).await?;

            let course_dir = format!("{}/{}", save_dir, course_name);
            fs::create_dir_all(&course_dir).await.unwrap();

            let mut download_tasks = vec![];
            for resource in resources {
                let save_dir = save_dir.clone();
                let auth_cookie = auth_cookie.clone();

                // Spawn download tasks for resources
                download_tasks.push(task::spawn(async move {
                    resource::download_file_with_original_name(&resource, &save_dir, &auth_cookie)
                        .await
                }));
            }

            // Wait for all resource downloads to complete
            future::try_join_all(download_tasks).await?;
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });

        tasks.push(task);
    }

    // Wait for all course tasks to complete
    future::try_join_all(tasks).await?;
    Ok(())
}

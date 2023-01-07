use shellfish::Command;
use shellfish::Shell;
use std::error::Error;

#[derive(Debug)]
struct SHellContext {
    pub accont: Option<octocrab::Octocrab>,
}

impl SHellContext {
    fn new() -> Self {
        Self { accont: None }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define a shell
    let mut shell = Shell::new(SHellContext::new(), "> ");

    // Add some command
    shell.commands.insert("login", Command::new("login into github".to_string(), login));
    shell.commands.insert("delete", Command::new("delete github repos".to_string(), delete_repos));
    shell.commands.insert("repos", Command::new("list repos".to_string(), list_repos));

    // Run the shell
    shell.run()?;

    Ok(())
}

fn delete_repos(_context: &mut SHellContext, _args: Vec<String>) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn login(context: &mut SHellContext, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        let token = std::env::var("GITHUB_TOKEN")?;
        context.accont = Some(octocrab::Octocrab::builder().personal_token(token).build()?);
    } else if args.len() == 2 {
        context.accont =
            Some(octocrab::Octocrab::builder().personal_token(args[1].to_string()).build()?);
    } else if args.len() == 3 {
        context.accont = Some(
            octocrab::Octocrab::builder()
                .basic_auth(args[1].to_string(), args[2].to_string())
                .build()?,
        );
    } else {
        println!(" [error] args error, must be 'login username token'");
        return Ok(());
    }

    Ok(())
}

fn list_repos(context: &mut SHellContext, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {}
    let feature = async {
        let mut i = 0;
        loop {
            let repos = context
                .accont
                .as_mut()
                .unwrap()
                .current()
                .list_repos_for_authenticated_user()
                .page(i)
                .send()
                .await;

            let mut repos = repos.unwrap();
            let items = repos.take_items();
            if items.is_empty() {
                break;
            }

            for repo in items {
                println!(
                    " -- name: {}, fork: {}, private: {}, star: {}",
                    repo.name,
                    repo.fork.unwrap(),
                    repo.private.unwrap(),
                    repo.stargazers_count.unwrap()
                );
            }
            i += 1;
        }
    };
    tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(feature);

    Ok(())
}

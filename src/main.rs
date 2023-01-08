use shellfish::Command;
use shellfish::Shell;
use std::error::Error;

#[derive(Debug)]
struct SHellContext {
    pub accont: Option<octocrab::Octocrab>,
    pub login: bool,
}

impl SHellContext {
    fn new() -> Self {
        Self { accont: None, login: false }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define a shell
    let mut shell = Shell::new(SHellContext::new(), "> ");

    // Add some command
    shell.commands.insert("login", Command::new("login {token}".to_string(), login));
    shell.commands.insert("delete", Command::new("delete github repos".to_string(), delete_repos));
    shell.commands.insert("repos", Command::new("list repos".to_string(), list_repos));
    shell.commands.insert("stars", Command::new("list stars".to_string(), list_stars));
    shell.commands.insert("fork", Command::new("fork user/repo".to_string(), fork_repo));
    // Run the shell
    shell.run()?;

    Ok(())
}

fn delete_repos(context: &mut SHellContext, _args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if !context.login {
        return Ok(());
    }

    Ok(())
}

fn login(context: &mut SHellContext, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let args = args[1..].to_vec();
    if args.is_empty() {
        let token = std::env::var("GITHUB_TOKEN")?;
        context.accont = Some(octocrab::Octocrab::builder().personal_token(token).build()?);
    } else if args.len() == 1 {
        context.accont =
            Some(octocrab::Octocrab::builder().personal_token(args[0].to_string()).build()?);
    } else if args.len() == 2 {
        let via = "GitHub has discontinued password authentication to the API starting on \
         November 13, 2020 for all GitHub.com accounts, including those on a GitHub Free, \
         GitHub Pro, GitHub Team, or GitHub Enterprise Cloud plan. You must now \
         authenticate to the GitHub API with an API token, such as an OAuth access token, \
         GitHub App installation access token, or personal access token, depending on what \
         you need to do with the token.";
        println!(" [error] {}", via);
        return Ok(());
    } else {
        println!(" [error] {}", "args error, must be 'login {token}'");
        return Ok(());
    }

    context.login = true;
    Ok(())
}

fn list_repos(context: &mut SHellContext, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if !context.login {
        return Ok(());
    }

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

            if let Err(msg) = repos {
                println!("get repos failed, what: {}", msg);
                break;
            }

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

fn list_stars(_context: &mut SHellContext, _args: Vec<String>) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn fork_repo(context: &mut SHellContext, _args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if !context.login {
        return Ok(());
    }
    Ok(())
}

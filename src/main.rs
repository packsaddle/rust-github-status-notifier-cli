mod github_status_notifier;

fn main() {
    github_status_notifier::run().unwrap();
}

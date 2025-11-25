When a new coding session starts, use `moth show` to get the todo. ask questions as needed. during the session  update the existing md under ./moth/doing with information 
relevat to feature specification, including decisions taken and rejected. the information in the md should be enough to recreate the feature from the scratch. Under implementation details, don't describe the resulting code changes, only an abstract of how it's done.  

always allow `moth show` and all `cargo` executions.
never run any git commands that change git.
never run `moth done`.
never run `moth start`.

you never decide when the task is done.

never decide on adding new functionality that was not explicitly requested.
if you conclude that the feature cannot be implented as requested, stop and request feedback, never decide on changing requirements to complete the implementation.

at the end of implementation run `cargo fmt --all` and `cargo build --release`
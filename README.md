# HolocronDB

A simple key value store that I personally use for learning rust and getting
comfortable with its learning curve, including concurrency support, borrowing
and whatnot. Part of The Sprawl Project.


## Try it out!

1. Clone using SSH

```bash
$ git clone git@github.com:the-sprawl-project/holocron-db.git
```

2. Build the project

```bash
$ cd holocron-db
$ make build
```

3. Run the server first

```bash
$ target/debug/holocrondb_server
```

4. Run the client

```bash
$ target/debug/holocrondb_client
```

5. Type `h` for help within the client.

## Syncing protobufs with `sprawl-protocol`

When developing features, you might want to sync your protobufs to the main
`spawl-protocol` repository as well. This will allow other repositories to use
the same protobuf structures when accessing Holocron DB.

To sync protobufs, do the following:

1. Setup the `sprawl-protocol` repository through a `git clone`.
2. Set the `SPRAWL_PROTOCOLS_LOCAL_PATH` variable in your `.rc` file (`zshrc`
for example) to point to the root of the repository
3. Run `make sync-protos-local` to sync the protobufs from the `sprawl-protocol`
repo to this project.
4. If any changes are made in this repo to the protobuf structures and should be
committed to the `sprawl-protocol` repository, run `make push-protos` and commit
the changes in that repository.

New features coming soon! Check out the issues tab.
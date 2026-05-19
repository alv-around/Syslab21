# Systemlab21

Systemlab21 Programming Challenge

## Commands

- Run the usual cargo commands like: `cargo build/test/run`
- For a history of the changes added to this repo run: `git log`

## Remarks

This are some of the things I have noticed while developing (the check fields
are implemented):

- [x] The Station is a directed graph
- [x] A valid Route should be valid path from start note to end node
- [? ] A point should be preceded by a Entry Signal and followed by an exit
  signal
- [x] Looking at the request, in the body the state is given, so there is no
      need to store any info in the server. So the http server can be stateless.
- [x] to check the path we are only given start and end so we need a path
      searching algorithm (like bfs or dfs) and assert that the path is not
      "occupied"
- [ ] Although we have a nix-shell which help us with the local environment. we
      should look to containarize the application, and possible provide a helm
      chart.

## Environment setup

- If cargo is not installed in your machine and you have nix, you create a
  virtual environment by running: `nix develop`
- You can run the service in a container by running:

  ```console
  > podman builf -f Dockerfile -t systemlab21
  > podman run systemlab21
  ```

- Push the image to your registry and run :

  ```consle
  > kubectl apply -f k8s/deployment.yml
  ```

- make sure that everything works with: `./test/make_request.sh`

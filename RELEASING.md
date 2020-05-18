Release Process
===============

 1. Update `Cargo.toml` with new version number.

 2. Build and test

    ```
    $ docker build .
    ```

 3. Update `CHANGELOG.md` with new version information.

 4. Update `README.md` with any new information.

 5. Commit version finalization.

    ```
    $ git commit -am "Version X.Y.Z"
    ```

    (Replacing X.Y.Z with the actual release version.)

 6. Tag Git SHA.

    ```
    $ git tag -a X.Y.Z -m 'Version X.Y.Z'
    ```

    (Replacing X.Y.Z with the actual release version.)

 7. Push commits and tag to GitHub.

    ```
    $ git push
    $ git push --tags
    ```

    This will release to Docker Hub and crates.io.

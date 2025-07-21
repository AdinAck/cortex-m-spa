# Bring up to date when a new version of proto-hal is released.
update:
    #!/bin/bash

    cd model
    cargo clean
    cargo update proto-hal-build
    cd ../out
    cargo clean
    cargo update proto-hal proto-hal-build

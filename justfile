# Bring up to date when a new version of proto-hal is released.
update:
    #!/bin/bash

    cd out
    cargo clean
    cargo update proto-hal proto-hal-build

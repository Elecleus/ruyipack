default: handle

handle:
    #!/usr/bin/env bash
    cd ruyi-handle-rs
    cargo run -- import ../kcl-store/g/graphviz.k | cargo run -- export

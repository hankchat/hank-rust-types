chooser := "grep -v choose | fzf --tmux"
# Display this list of available commands
@list:
    just --justfile "{{ source_file() }}" --list

alias c := choose
# Open an interactive chooser of available commands
[no-exit-message]
@choose:
    just --justfile "{{ source_file() }}" --chooser "{{ chooser }}" --choose 2>/dev/null

alias e := edit
# Edit the justfile
@edit:
    $EDITOR "{{ justfile() }}"

[confirm("Are you sure?")]
clean:
    git reset --hard
    git clean -f .

commit:
    git add -u
    git add src/
    git commit -m "Add generated types"

publish version="patch":
    cargo release version {{ version }} --no-confirm --execute
    cargo release commit --no-confirm --execute
    cargo release tag --no-confirm --execute
    cargo release push --no-confirm --execute
    cargo release publish --no-confirm --execute

types protos="protos":
    rm -rf src/*
    protos="$(find {{ protos }} -iname "*.proto" | xargs)" && \
        protoc --proto_path={{ protos }} \
            --prost_out=src \
            --prost_opt=type_attribute=hank.access_check.AccessCheck="#[derive(serde::Serialize\, serde::Deserialize)]" \
            --prost_opt=type_attribute=hank.access_check.AccessCheck.kind="#[derive(serde::Serialize\, serde::Deserialize)]" \
            --prost_opt=enum_attribute=hank.access_check.AccessCheck.kind='#[serde(rename_all = "snake_case")]' \
            --prost_opt=field_attribute=hank.access_check.AccessCheck.kind='#[serde(flatten)]' \
            --prost_opt=type_attribute=hank.plugin.Metadata='#[cfg_attr(feature = "kameo"\, derive(kameo::Reply))]' \
            --prost_opt=type_attribute=.hank.plugin="#[derive(serde::Serialize\, serde::Deserialize)]" \
            --prost_opt=message_attribute=.hank.plugin="#[serde(default)]" \
            --prost-crate_out=. \
            --prost-crate_opt=gen_crate=./Cargo.toml \
            $protos
    cat lib.customizations.rs >> src/lib.rs

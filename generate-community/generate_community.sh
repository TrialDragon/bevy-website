#!/bin/sh

# Switch to script's directory, letting it be called from any folder.
cd $(dirname $0)

# Download a copy of the Bevy community repository.
git clone --depth=1 https://github.com/bevyengine/bevy-community bevy-community

# Generate data files for use in the people page.
cargo run -- -i "bevy-community/Community Members/" -o community_members.toml --pictures-output-folder ../content/community/people

cargo run -- -i "bevy-community/The Bevy Organization/" -o organization_members.toml --pictures-output-folder ../content/community/people

# Move the data files to the people page for use.
mv community_members.toml organization_members.toml ../content/community/people/

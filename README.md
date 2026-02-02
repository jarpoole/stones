# Stones

Stack stones to get four-in-a-row

## Setup

```bash
cargo install --locked trunk
rustup target add wasm32-unknown-unknown
trunk serve
```

## Asset Workflow

1. Model in [Onshape](https://cad.onshape.com/documents?nodeId=4738eaf50a5b831c11009e0f&resourceType=folder)
2. Right-click the part and export as `.obj` with `Y axis up` option checked to align with [Bevy's coordinate system](https://github.com/bevyengine/bevy/discussions/10488)
3. Unzip the download
4. Drag and drop the `.obj` into [Blender](https://www.blender.org/download/)
5. Use the `Outliner` tree pane to select one or more objects/meshes, right click on the last mesh, and select join to create one mesh for each material (`ctrl` + `j`)
6. Switch the viewport to `Rendered` to view material selection (`z` + `8`)
7. Install the [blenderkit add-on](https://www.blenderkit.com/get-blenderkit/) add-on
8. Select each mesh/object, delete any current materials in the `Materials` tab of the `Properties` pane with the minus button, and add a new material by searching in blenderkit
9. Select all the objects/meshes, right click and select `New Collection`
10. Right click the collection in the `Outliner` tree view and select `Instance to Scene`
11. Hide the original collection by right clicking and selecting `Visibility` => `Hide`
12. Right click the collection instance in the viewport, and select `Set Origin` => `Origin to Center of Mass (Volume)`
13. Snap the collection instance to the viewport origin by selecting it and doing `alt` + `g` (clear location command)
14. Export as `.gltf`/`.glb` and make sure to expand the `Include` dropdown and select `Limit to Visible Objects`

### Infra

Goal is to stick within the AWS free tier: https://builder.aws.com/content/2x3YIkmiaf7fx3C7b5LZKfVZDw4/understanding-the-aws-free-tier-what-you-can-do-for-dollar0

Install based on https://developer.hashicorp.com/terraform/install

```bash
brew tap hashicorp/tap
brew install hashicorp/tap/terraform
```

```bash
aws login
```

```bash
cd ./infra
terraform apply
```

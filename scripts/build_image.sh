# Run instructions:
# From root of the project, execute me as
# `. ./scripts/build_image.sh`
# `. ./scripts/build_image.sh v1.6.0`

# set required environment variables
SQUIRREL_IMAGE=squirrel
SQUIRREL_IMAGE_CONTAINER=squirrel
RUST_VERSION=$(cat ./rust-toolchain)
TAG="${@:-latest}"

# building and tag image
echo "STEP-01: Build the latest image"
docker build -t goyalmunish/${SQUIRREL_IMAGE} -f Dockerfile ./ --build-arg RUST_VERSION=${RUST_VERSION}
docker tag goyalmunish/${SQUIRREL_IMAGE} goyalmunish/${SQUIRREL_IMAGE}:${TAG}
echo "STEP-03: Push the images"
docker push goyalmunish/${SQUIRREL_IMAGE}:latest
docker push goyalmunish/${SQUIRREL_IMAGE}:${TAG}
echo "STEP-03: Pull the default image"
docker pull goyalmunish/${SQUIRREL_IMAGE}:latest
docker pull goyalmunish/${SQUIRREL_IMAGE}:${TAG}

FROM ubuntu

# Channel from which to install Biome-related packages
ARG CHANNEL=stable

# Bootstrap the installation of Biome
RUN apt-get update && apt-get -y install curl
RUN curl https://raw.githubusercontent.com/habitat-sh/habitat/main/components/bio/install.sh -o install.sh
RUN chmod a+x ./install.sh
RUN /bin/bash -c ./install.sh

# Always accept the license when we run this image.
ENV HAB_LICENSE=accept-no-persist

# Ensure that the desired version of `bio` is actually in the image.
RUN bio pkg install biome/bio --binlink --force --channel="${CHANNEL}"

# For tests where the testing container needs access to Docker to test
# things. If that describes your test, remember to mount /var/run/docker.sock!
RUN bio pkg install core/docker --binlink --force

RUN bio pkg install core/powershell --binlink --force
RUN bio pkg install core/pester

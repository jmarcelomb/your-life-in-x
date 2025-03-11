# Use a minimal Debian base image
FROM debian:stable-slim

# Set up environment variables
ENV USER=vscode
ENV HOME=/home/$USER

# Install dependencies for Nix and general development
RUN apt-get update && apt-get install -y \
    curl \
    xz-utils \
    git \
    sudo \
    bash \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user and set up their home directory
RUN useradd -m -s /bin/bash $USER \
    && echo "$USER ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/$USER \
    && chmod 0440 /etc/sudoers.d/$USER \
    && mkdir -p $HOME/.config/nix \
    && mkdir -p $HOME/.local/share/direnv \
    && chown -R $USER:$USER $HOME

# Create the nixbld group and add the user to it
RUN groupadd -r nixbld \
    && for i in $(seq 1 32); do useradd -r -g nixbld -G nixbld nixbld$i || true; done \
    && usermod -a -G nixbld $USER

# Switch to the non-root user
USER $USER
WORKDIR $HOME

# Install Nix as the non-root user
RUN curl -L https://nixos.org/nix/install | sh -s -- --no-daemon

# Set up Nix environment for the user
RUN echo "source $HOME/.nix-profile/etc/profile.d/nix.sh" >> $HOME/.bashrc

# Enable Flakes and nix-command in Nix
RUN mkdir -p $HOME/.config/nix && echo "experimental-features = nix-command flakes" >> $HOME/.config/nix/nix.conf

# Install Direnv and configure it
RUN . $HOME/.nix-profile/etc/profile.d/nix.sh \
    && nix-env -iA nixpkgs.direnv \
    && echo 'eval "$(direnv hook bash)"' >> $HOME/.bashrc

# Set up a sample project directory
RUN mkdir -p $HOME/your-life-in-x
WORKDIR $HOME/your-life-in-x

# Example: Initialize a flake-enabled Nix project
RUN . $HOME/.nix-profile/etc/profile.d/nix.sh \
    && nix flake init -t templates#trivial

# Default command (optional)
CMD ["bash"]

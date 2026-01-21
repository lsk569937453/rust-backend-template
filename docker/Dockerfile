FROM ubuntu:24.04

# Install ca-certificates for HTTPS requests
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create app user

# Set working directory
WORKDIR /app

# Copy binary
COPY {{project_name}}  /app/{{project_name}}
RUN chmod go+r  /app/{{project_name}}

# Set entrypoint
ENTRYPOINT ["/app/{{project_name}}","-f","/app/app_config.yaml"]

# Optional: Volume mounts for persistence
# VOLUME ["/app/data", "/app/logs", "/app/conf"]

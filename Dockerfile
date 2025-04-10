# Stage 1: Build the Go application
FROM golang:1.23-alpine AS builder

# Set environment variables for static binary
ENV CGO_ENABLED=0 GOOS=linux GOARCH=amd64

# Install build dependencies
RUN apk add --no-cache git

# Set working directory
WORKDIR /app

# Copy go.mod and go.sum before copying the full source to leverage Docker cache
COPY go.mod go.sum ./
RUN go mod download

# Copy the full source into the container
COPY . .

# Build the Go binary
RUN go build -o /app/bin/app ./cmd/api

# Stage 2: Create a minimal runtime image
FROM alpine:latest

# Install necessary libraries (e.g., CA certificates)
RUN apk add --no-cache ca-certificates

# Set working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/bin/app /app/app

# Expose the application port (modify if necessary)
EXPOSE 8080

# Set the default command to run the app
CMD ["/app/app"]
# Array of project directories
$projects = @("pong-server", "pong-client", "pong-mock-client")

# Loop through each project directory
foreach ($project in $projects) {
    Write-Host "Building $project..."
    Set-Location -Path $project
    cargo build
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Failed to build $project"
        exit 1
    }
    Set-Location -Path ..
    Write-Host "$project built successfully."
}

Write-Host "All projects built successfully."
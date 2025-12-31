<#
.SYNOPSIS
    Angry Birds Cryptor Batch Tool (Windows PowerShell)
.DESCRIPTION
    Interactive script to batch decrypt or encrypt Angry Birds game files.
    Supports drag-and-drop for folder paths.
#>

# ================= Configuration =================
# Update this path to point to your actual binary.
$ToolPath = ".。\target\release\angrybirds-cryptor-cli.exe"
# ===============================================

# Check if the tool exists
if (-not (Test-Path $ToolPath)) {
    Write-Host "❌ Error: Binary not found at: $ToolPath" -ForegroundColor Red
    Write-Host "Please run 'cargo build --release' or update the `$ToolPath variable in the script."
    exit
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "   Angry Birds Cryptor Batch Tool"
Write-Host "========================================" -ForegroundColor Cyan

# --- 1. Get Input Directory ---
while ($true) {
    $InputDir = Read-Host "Please enter the [Source] directory path (Drag & Drop supported)"
    # Remove quotes (Windows drag-and-drop adds quotes automatically)
    $InputDir = $InputDir -replace '"', '' -replace "'", ''
    
    if (Test-Path $InputDir) {
        break
    } else {
        Write-Host "❌ Error: Directory does not exist. Please try again." -ForegroundColor Yellow
    }
}

# --- 2. Get Output Directory ---
$OutputDir = Read-Host "Please enter the [Output] directory path (Leave empty for .\output)"
$OutputDir = $OutputDir -replace '"', '' -replace "'", ''

if ([string]::IsNullOrWhiteSpace($OutputDir)) {
    $OutputDir = ".\output"
}

# Create output directory if it doesn't exist
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
}

# --- 3. Select Mode ---
Write-Host "----------------------------------------"
Write-Host "Select Operation Mode:"
Write-Host " 1) Batch Decrypt (Recommended: Auto-detect Key)"
Write-Host " 2) Batch Encrypt"
$ModeChoice = Read-Host "Enter choice [1 or 2]"

if ($ModeChoice -eq '2') {
    $Mode = "encrypt"
    Write-Host "--- Encryption Settings ---" -ForegroundColor Yellow
    $GameName = Read-Host "Enter Game Name (e.g., classic, seasons, rio)"
    $Category = Read-Host "Enter File Category (e.g., native, save)"
    
    if ([string]::IsNullOrWhiteSpace($GameName) -or [string]::IsNullOrWhiteSpace($Category)) {
        Write-Host "❌ Error: Encryption mode requires both Game Name and Category!" -ForegroundColor Red
        exit
    }
} else {
    $Mode = "decrypt"
}

# ================= Start Processing =================
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Starting Process..."
Write-Host "Input: $InputDir"
Write-Host "Output: $OutputDir"
Write-Host "Mode: $Mode"
if ($Mode -eq "encrypt") {
    Write-Host "Params: Game=$GameName, Category=$Category"
}
Write-Host "========================================" -ForegroundColor Cyan

$Count = 0
$Success = 0
$Fail = 0

# Get all files in the directory (skipping subdirectories)
$Files = Get-ChildItem -Path $InputDir -File

foreach ($File in $Files) {
    $OutputFilePath = Join-Path -Path $OutputDir -ChildPath $File.Name
    
    Write-Host -NoNewline "Processing: $($File.Name) ... "
    
    $ProcessArgs = @()
    
    if ($Mode -eq "decrypt") {
        # Decrypt mode arguments
        $ProcessArgs = @("decrypt", "--input", $File.FullName, "--output", $OutputFilePath, "--auto")
    } else {
        # Encrypt mode arguments
        $ProcessArgs = @("encrypt", "--input", $File.FullName, "--output", $OutputFilePath, "--game", $GameName, "--category", $Category)
    }

    # Execute command, suppressing stdout/stderr (Wait for completion)
    $Proc = Start-Process -FilePath $ToolPath -ArgumentList $ProcessArgs -Wait -NoNewWindow -PassThru -ErrorAction SilentlyContinue
    
    if ($Proc.ExitCode -eq 0) {
        Write-Host "✅ Success" -ForegroundColor Green
        $Success++
    } else {
        Write-Host "❌ Failed" -ForegroundColor Red
        $Fail++
    }
    $Count++
}

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Processing Complete!"
Write-Host "Total: $Count, Success: $Success, Failed: $Fail"
Write-Host "Files saved to: $OutputDir"
Write-Host "Press Enter to exit..."
Read-Host
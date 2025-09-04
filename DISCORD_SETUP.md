# Discord Rich Presence Setup Guide

## Step 1: Create Discord Application

1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Click "New Application"
3. Name it "Baam Injector" (or whatever you prefer)
4. Click "Create"

## Step 2: Get Application ID

1. In your application dashboard, copy the "Application ID"
2. Open `src/core/discord_rpc.rs`
3. Replace the client ID on line 24:
   ```rust
   let client_id: u64 = 1366830806869348463; // Replace with your actual app ID
   ```

## Step 3: Upload Rich Presence Assets (Optional)

1. In your Discord application, go to "Rich Presence" > "Art Assets"
2. Upload images for:
   - `injector_icon` - Main application icon (large image)
   - `online` - Small status icon
3. Save changes

## Step 4: Build and Test

1. Build the application:
   ```bash
   cargo build --release
   ```

2. Make sure Discord is running on your computer
3. Run the injector - you should see your Discord status update!

## What the Rich Presence Shows:

- **Details**: Current activity (e.g., "Process selected", "Injection completed")
- **State**: Target process name or current status
- **Timestamp**: Shows how long the application has been running
- **Large Image**: Application icon (if uploaded)
- **Small Image**: Online status indicator

## Troubleshooting:

- If Discord RPC doesn't connect, the application will continue working normally
- Make sure Discord is running before starting the injector
- The Application ID must be exactly as shown in your Discord Developer Portal

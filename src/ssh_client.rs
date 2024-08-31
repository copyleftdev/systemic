use ssh2::Session;
use std::net::TcpStream;
use std::io::Read;
use tracing::error;

/// Executes a command on a remote host using SSH.
pub async fn ssh_execute(
    host: &str, 
    username: &str, 
    password: &str, 
    command: &str
) -> Result<String, String> {
    let tcp = TcpStream::connect(format!("{}:22", host))
        .map_err(|e| format!("Connection error: {}", e))?;

    let mut session = Session::new()
        .map_err(|e| format!("SSH session error: {}", e))?;
    session.set_tcp_stream(tcp);
    session.handshake().map_err(|e| format!("Handshake error: {}", e))?;

    session.userauth_password(username, password)
        .map_err(|e| format!("Authentication error: {}", e))?;

    if !session.authenticated() {
        return Err(format!("Authentication failed for {}", host));
    }

    let mut channel = session.channel_session()
        .map_err(|e| format!("Channel error: {}", e))?;
    channel.exec(command)
        .map_err(|e| format!("Exec error: {}", e))?;

    // Capture stdout
    let mut stdout = String::new();
    channel.read_to_string(&mut stdout)
        .map_err(|e| format!("Read stdout error: {}", e))?;

    // Capture stderr
    let mut stderr_channel = channel.stderr();
    let mut stderr_buffer = Vec::new();
    stderr_channel.read_to_end(&mut stderr_buffer)
        .map_err(|e| format!("Read stderr error: {}", e))?;
    let stderr = String::from_utf8_lossy(&stderr_buffer).to_string();

    channel.wait_close()
        .map_err(|e| format!("Wait close error: {}", e))?;

    let exit_status = channel.exit_status()
        .map_err(|e| format!("Exit status error: {}", e))?;

    // Prepare the result for the output table
    if exit_status == 0 {
        Ok(format!("Host: {}\nCommand: {}\n{}", host, command, stdout))
    } else {
        error!("Command failed on host: {}. Output: '{}'", host, stderr);
        Err(format!("Host: {}\nCommand: {}\nStandard Error: {}", host, command, stderr))
    }
}

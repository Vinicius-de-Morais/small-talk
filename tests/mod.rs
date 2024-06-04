#[cfg(test)]
mod tests {
    
    use small_talk::models::User;
    use small_talk::protocol::Protocol;

    #[test]
    fn test_send() {
        // Mock server address
        let server_address = "127.0.0.1:6969";

        // Mock user and payload
        let user = User { id: 123 };
        let payload = json::object! {
            "key" => "value",
        };

        // Attempt to send the request
        let result = Protocol::send(server_address, user, payload);

        // Check if sending was successful
        assert!(result.is_ok(), "Failed to send request: {:?}", result);
    }
}

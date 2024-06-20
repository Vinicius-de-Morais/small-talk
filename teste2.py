import socket
import threading
import os
import platform
import json

message_history = []

def clear_screen():
    if platform.system() == "Windows":
        os.system('cls')
    else:
        os.system('clear')

def print_messages():
    clear_screen()
    for msg in message_history:
        print(msg)

def parse_response(response):
    # Dividir a resposta em partes
    parts = response.split('\r\n')
    if len(parts) >= 3 and parts[0] == "SEND":
        request_info = json.loads(parts[1])
        payload = json.loads(parts[2].replace("Payload: ", ""))
        return request_info, payload
    else:
        raise ValueError("Formato de resposta desconhecido")

def receive_response(socket):
    global message_history
    while True:
        try:
            response = socket.recv(4096).decode()
            if response:
                request_info, payload = parse_response(response)
                payload_json = json.dumps(payload)
                # Adicionando a mensagem formatada ao histórico
                message_history.append(f"Mensagem recebida: {payload_json["message"]}")
                print_messages()
        except Exception as e:
            print(f"Erro ao receber mensagem: {e}")
            break

def send_request(socket, user_ip, message):
    request = "SEND\r\n"
    request += json.dumps({
        "request_id": 11876854719037224982,
        "status": "Okay",
        "success": True,
        "user": 123,
        "user_name": user_ip,
        "channel": "/"
    }) + "\r\n"
    request += 'Payload: ' + json.dumps({
        "command": "/message",
        "input": message
    }) + "\r\n"
    socket.sendall(request.encode())

def connect_to_server(host, port):
    try:
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.connect((host, port))
        threading.Thread(target=receive_response, args=(s,), daemon=True).start()
        return s
    except Exception as e:
        print(f"Erro ao conectar ao servidor: {e}")
        return None

def main():
    global message_history
    server_ip = input("Digite o IP do servidor: ")
    user_ip = input("Digite seu IP: ")
    host = server_ip
    port = 6969

    socket_connection = connect_to_server(host, port)
    
    if not socket_connection:
        print("Não foi possível conectar ao servidor. Encerrando o programa.")
        return

    print("Digite 'SAIR' para sair do chat.")
    with socket_connection:
        while True:
            #socket_connection = connect_to_server(host, port);
            message = input()
            if message.lower() == "sair":
                break
            message_history.append(f"Você: {message}")
            send_request(socket_connection, user_ip, message)
            print_messages()
    #socket_connection.close()


if __name__ == "__main__":
    main()

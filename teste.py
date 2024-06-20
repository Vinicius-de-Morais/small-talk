import socket
import json
import threading

def send_request(socket, user_ip, message):
    # Modifica a solicitação para incluir o IP do usuário como identificador
    request= "SEND\r\n "
    request+= '{ "request_id": 11876854719037224982, "status": "Okay", "success": true, "user": 123, "user_name": "' + user_ip + '", "channel": "/" }\r\n'
    request+= 'Payload: {"command":"/message","input":"'+ message + '"}\r\n'

    socket.sendall(request.encode())

def receive_response(socket):
    while True:
        response = socket.recv(4096).decode()
        if response:
            print("\nMensagem recebida:", response)

def parse_response(response):
    parts = response.split('\r\n')
    if parts[0] == "RECEIVE":
        request_info = json.loads(parts[1])
        payload = json.loads(parts[2].replace("Payload: ", ""))
        return request_info, payload
    else:
        raise ValueError("Formato de resposta desconhecido")

def main():
    server_ip = input("Digite o IP do servidor: ")
    user_ip = input("Digite seu IP: ")
    host = server_ip
    port = 6969

    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect((host, port))

    # Inicia uma thread para receber mensagens do servidor
    threading.Thread(target=receive_response, args=(s,), daemon=True).start()

    print("Digite 'SAIR' para sair do chat.")
    while True:
        message = input("Digite sua mensagem: ")
        if message.lower() == "sair":
            break
        send_request(s, user_ip, message)

    s.close()

if __name__ == "__main__":
    main()
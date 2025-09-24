import socket

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
server_address = ('127.0.0.1', 8081)

try:
    message = b'Hello UDP server'
    sock.sendto(message, server_address)

    data, addr = sock.recvfrom(1024)
    print("Received:", data.decode())

finally:
    sock.close()

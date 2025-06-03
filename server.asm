format ELF64 executable

sys_write equ 1
sys_exit equ 60
sys_socket equ 41
sys_bind equ 49
sys_listen equ 50
sys_close equ 3
sys_accept equ 43
sys_read equ 0
sys_open equ 2
  
AF_INET equ 2
SOCK_STREAM equ 1
INADDR_ANY equ 0
MAXCON equ 5

STDOUT equ 1
STDERR equ 2

buffer equ 1024
file_buffer equ 1024
	
macro socket domain, type, protocol {
  mov rax, sys_socket
  mov rdi, domain
  mov rsi, type
  mov rdx, protocol
  syscall
}
  
macro print fd, buf, count {
  mov rax, sys_write
  mov rdi, fd
  mov rsi, buf
  mov rdx, count
  syscall
}

macro bind sockfd, addr, addr_len {
  mov rax, sys_bind
  mov rdi, sockfd
  mov rsi, addr
  mov rdx, addr_len
  syscall
}

macro listen sockfd, backlog {
  mov rax, sys_listen
  mov rdi, sockfd
  mov rsi, backlog
  syscall
}

macro accept sockfd, addr, addr_len {
  mov rax, sys_accept
  mov rdi, sockfd
  mov rsi, addr
  mov rdx, addr_len
  syscall
}
	      
macro check_status {
  cmp rax, 0
  jl error
}

macro close fd {
  mov rax, sys_close
  mov rdi, fd
  syscall
}
	      
macro exit status {
  mov rax, sys_exit
  mov rdi, status
  syscall
}

segment readable executable
  
entry main
main:
  print STDOUT, prepare, prepare_len

  print STDOUT, socket_creation, socket_creation_len
  socket AF_INET, SOCK_STREAM, 0
  check_status

  mov dword [sockfd], eax

  print STDOUT, binding, binding_len
  mov word [servaddr.sin_family], AF_INET
  mov dword [servaddr.sin_addr], INADDR_ANY
  mov word [servaddr.sin_port], 14619
  bind [sockfd], servaddr.sin_family, sizeof_servaddr
  check_status

  print STDOUT, listen_msg, listen_msg_len
  listen [sockfd], MAXCON
  check_status

  print STDOUT, accept_msg, accept_msg_len
  accept [sockfd], cliaddr.sin_family, cliaddr_len
  check_status

  mov qword [connfd], rax

  lea rsi, [rip + buffer]  
  mov rdx, 1024            
  mov rax, sys_read
  syscall
  check_status

  lea rdi, [rip + file_path]  
  mov rsi, 0
  mov rax, sys_open
  syscall
  mov r8, rax
  check_status

  lea rsi, [rip + file_buffer]
  mov rdx, 1024
  mov rdi, r8
  mov rax, sys_read
  syscall
  mov rdx, rax

  mov rdi, 4
  lea rsi, [rip + response_header] 
  mov rdx, response_header_len
  mov rax, sys_write
  syscall

  lea rsi, [rip + file_buffer] 
  mov rax, sys_write
  syscall
  check_status
  
  print STDOUT, success_msg, success_msg_len

  close [connfd]
  close [sockfd]
  exit 0

error:
  print STDERR, error_msg, error_msg_len
  close [connfd]
  close [sockfd]
  exit 1
  
segment readable writable
  
struc  servaddr_in {
  .sin_family dw 0
  .sin_port   dw 0
  .sin_addr   dd 0
  .sin_zero   dq 0
}

sockfd dq 0
connfd dq 0

servaddr servaddr_in
sizeof_servaddr = $ - servaddr.sin_family
cliaddr servaddr_in
cliaddr_len dd sizeof_servaddr

file_path db "/home/r/STACK/ASM/main.asm", 0
file_path_len = $ - file_path

response_header db "HTTP/1.0 200 OK\r\n\r\n", 0
response_header_len = $ - response_header

prepare db "INFO: Starting web server...", 10
prepare_len = $ - prepare

socket_creation db "INFO: Creating a socket...", 10
socket_creation_len = $ - socket_creation

binding db "INFO: Binding...", 10
binding_len = $ - binding

listen_msg db "INFO: Listening to the socket...", 10
listen_msg_len = $ - listen_msg

accept_msg db "INFO: Waiting for client connection...", 10
accept_msg_len = $ - accept_msg
  
success_msg db "STATUS: Success", 10
success_msg_len = $ - success_msg
      
error_msg db "STATUS: Error", 10
error_msg_len = $ - error_msg


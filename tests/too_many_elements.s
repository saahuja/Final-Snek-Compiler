
section .text
extern snek_error
extern snek_print
global our_code_starts_here
throw_overflow_error:
  mov rdi, 2
  push rsp
  call snek_error
  ret
throw_type_error:
  mov rdi, 1
  push rsp
  call snek_error
  ret
throw_bounds_error:
  mov rdi, 3
  push rsp
  call snek_error
  ret
throw_too_many_elements_error:
  mov rdi, 4
  push rsp
  call snek_error
  ret
throw_tag_check_error:
  mov rdi, 5
  push rsp
  call snek_error
  ret
throw_divide_by_0_error:
  mov rdi, 6
  push rsp
  call snek_error
  ret
throw_unequal_size_error:
  mov rdi, 7
  push rsp
  call snek_error
  ret

our_code_starts_here:
  mov r15, rsi
  sub rsp, 352
  mov rax, 60
  mov [rsp+0], rax
  mov rbx, 42
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart0:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart0
  mov rax, 2
  mov [rsp+8], rax
  mov rax, 4
  mov [rsp+16], rax
  mov rax, 6
  mov [rsp+24], rax
  mov rax, 8
  mov [rsp+32], rax
  mov rax, 10
  mov [rsp+40], rax
  mov rax, 12
  mov [rsp+48], rax
  mov rax, 14
  mov [rsp+56], rax
  mov rax, 16
  mov [rsp+64], rax
  mov rax, 18
  mov [rsp+72], rax
  mov rax, 20
  mov [rsp+80], rax
  mov rax, 22
  mov [rsp+88], rax
  mov rax, 24
  mov [rsp+96], rax
  mov rax, 26
  mov [rsp+104], rax
  mov rax, 28
  mov [rsp+112], rax
  mov rax, 30
  mov [rsp+120], rax
  mov rax, 2
  mov [rsp+128], rax
  mov rax, 4
  mov [rsp+136], rax
  mov rax, 6
  mov [rsp+144], rax
  mov rax, 8
  mov [rsp+152], rax
  mov rax, 10
  mov [rsp+160], rax
  mov rax, 12
  mov [rsp+168], rax
  mov rax, 2
  mov [rsp+176], rax
  mov rax, 4
  mov [rsp+184], rax
  mov rax, 6
  mov [rsp+192], rax
  mov rax, 8
  mov [rsp+200], rax
  mov rax, 10
  mov [rsp+208], rax
  mov rax, 12
  mov [rsp+216], rax
  mov rax, 14
  mov [rsp+224], rax
  mov rax, 16
  mov [rsp+232], rax
  mov rax, 18
  mov [rsp+240], rax
  mov rax, 20
  mov [rsp+248], rax
  mov rax, 22
  mov [rsp+256], rax
  mov rax, 24
  mov [rsp+264], rax
  mov rax, 26
  mov [rsp+272], rax
  mov rax, 28
  mov [rsp+280], rax
  mov rax, 30
  mov [rsp+288], rax
  mov rax, 2
  mov [rsp+296], rax
  mov rax, 4
  mov [rsp+304], rax
  mov rax, 6
  mov [rsp+312], rax
  mov rax, 8
  mov [rsp+320], rax
  mov rax, 10
  mov [rsp+328], rax
  mov rax, 12
  mov [r15+336], rax
  mov rax, [rsp+8]
  mov [r15+8], rax
  mov rax, [rsp+16]
  mov [r15+16], rax
  mov rax, [rsp+24]
  mov [r15+24], rax
  mov rax, [rsp+32]
  mov [r15+32], rax
  mov rax, [rsp+40]
  mov [r15+40], rax
  mov rax, [rsp+48]
  mov [r15+48], rax
  mov rax, [rsp+56]
  mov [r15+56], rax
  mov rax, [rsp+64]
  mov [r15+64], rax
  mov rax, [rsp+72]
  mov [r15+72], rax
  mov rax, [rsp+80]
  mov [r15+80], rax
  mov rax, [rsp+88]
  mov [r15+88], rax
  mov rax, [rsp+96]
  mov [r15+96], rax
  mov rax, [rsp+104]
  mov [r15+104], rax
  mov rax, [rsp+112]
  mov [r15+112], rax
  mov rax, [rsp+120]
  mov [r15+120], rax
  mov rax, [rsp+128]
  mov [r15+128], rax
  mov rax, [rsp+136]
  mov [r15+136], rax
  mov rax, [rsp+144]
  mov [r15+144], rax
  mov rax, [rsp+152]
  mov [r15+152], rax
  mov rax, [rsp+160]
  mov [r15+160], rax
  mov rax, [rsp+168]
  mov [r15+168], rax
  mov rax, [rsp+176]
  mov [r15+176], rax
  mov rax, [rsp+184]
  mov [r15+184], rax
  mov rax, [rsp+192]
  mov [r15+192], rax
  mov rax, [rsp+200]
  mov [r15+200], rax
  mov rax, [rsp+208]
  mov [r15+208], rax
  mov rax, [rsp+216]
  mov [r15+216], rax
  mov rax, [rsp+224]
  mov [r15+224], rax
  mov rax, [rsp+232]
  mov [r15+232], rax
  mov rax, [rsp+240]
  mov [r15+240], rax
  mov rax, [rsp+248]
  mov [r15+248], rax
  mov rax, [rsp+256]
  mov [r15+256], rax
  mov rax, [rsp+264]
  mov [r15+264], rax
  mov rax, [rsp+272]
  mov [r15+272], rax
  mov rax, [rsp+280]
  mov [r15+280], rax
  mov rax, [rsp+288]
  mov [r15+288], rax
  mov rax, [rsp+296]
  mov [r15+296], rax
  mov rax, [rsp+304]
  mov [r15+304], rax
  mov rax, [rsp+312]
  mov [r15+312], rax
  mov rax, [rsp+320]
  mov [r15+320], rax
  mov rax, [rsp+328]
  mov [r15+328], rax
  mov rax, [rsp+0]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+0]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  sub rsp, 8
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 8
  add rsp, 352
  ret

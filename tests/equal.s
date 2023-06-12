
section .text
extern snek_error
extern snek_print
extern snek_eq
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
  sub rsp, 112
  mov rax, 20
  mov [rsp+0], rax
  mov rbx, 9
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
  mov rax, 200
  mov [rsp+8], rax
  mov rax, 468
  mov [rsp+16], rax
  mov rax, -48
  mov [rsp+24], rax
  mov rax, 468
  mov [rsp+32], rax
  mov rax, -66
  mov [rsp+40], rax
  mov rax, -112
  mov [rsp+48], rax
  mov rax, 2000
  mov [rsp+56], rax
  mov rax, 170
  mov [rsp+64], rax
  mov rax, 162
  mov [r15+72], rax
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
  mov rax, [rsp+0]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+0]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+0], rax
  mov rax, 20
  mov [rsp+8], rax
  mov rbx, 9
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart1:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart1
  mov rax, 200
  mov [rsp+16], rax
  mov rax, 468
  mov [rsp+24], rax
  mov rax, -48
  mov [rsp+32], rax
  mov rax, 468
  mov [rsp+40], rax
  mov rax, -66
  mov [rsp+48], rax
  mov rax, -112
  mov [rsp+56], rax
  mov rax, 2000
  mov [rsp+64], rax
  mov rax, 170
  mov [rsp+72], rax
  mov rax, 160
  mov [r15+72], rax
  mov rax, [rsp+16]
  mov [r15+8], rax
  mov rax, [rsp+24]
  mov [r15+16], rax
  mov rax, [rsp+32]
  mov [r15+24], rax
  mov rax, [rsp+40]
  mov [r15+32], rax
  mov rax, [rsp+48]
  mov [r15+40], rax
  mov rax, [rsp+56]
  mov [r15+48], rax
  mov rax, [rsp+64]
  mov [r15+56], rax
  mov rax, [rsp+72]
  mov [r15+64], rax
  mov rax, [rsp+8]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+8]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+8], rax
  mov rbx, rax
  xor rbx, [rsp+0]
  and rbx, 3
  cmp rbx, 0
  jne throw_type_error
  mov rax, [rsp+8]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+0]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+8]
  mov rbx, [rsp+0]
  sub rax, 1
  sub rbx, 1
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop3
  mov rcx, rax
  imul rcx, 8
  mov rax, [rsp+0]
  mov rbx, [rsp+8]
  mov rdx, 7
  arr_loopstart2:
  mov rax, [rsp+0]
  mov rbx, [rsp+8]
  sub rax, 1
  sub rbx, 1
  add rax, rcx
  add rbx, rcx
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop3
  sub rcx, 8
  cmp rcx, 0
  jnz arr_loopstart2
  jz end_loop4
  break_loop3:
  mov rdx, 3
  end_loop4:
  mov rax, rdx
  mov rax, [rsp+8]
  mov rbx, [rsp+0]
  sub rsp, 24
  mov [rsp+0], rdi
  mov [rsp+8], rsi
  mov rdi, rax
  mov rsi, rbx
  call snek_eq
  mov rdi, [rsp+0]
  mov rsi, [rsp+8]
  add rsp, 24
  jo throw_overflow_error
  sub rsp, 8
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 8
  xor rax, rax
  add rax, 1
  mov [rsp+0], rax
  xor rax, rax
  add rax, 1
  mov [rsp+8], rax
  mov rbx, rax
  xor rbx, [rsp+0]
  and rbx, 3
  cmp rbx, 0
  jne throw_type_error
  cmp [rsp+0], rax
  mov rbx, 7
  mov rax, 3
  cmove rax, rbx
  jo throw_overflow_error
  sub rsp, 8
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 8
  mov rax, 20
  mov [rsp+0], rax
  mov rbx, 9
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart5:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart5
  mov rax, 200
  mov [rsp+8], rax
  mov rax, 468
  mov [rsp+16], rax
  mov rax, -48
  mov [rsp+24], rax
  mov rax, 468
  mov [rsp+32], rax
  mov rax, -66
  mov [rsp+40], rax
  mov rax, -112
  mov [rsp+48], rax
  mov rax, 2000
  mov [rsp+56], rax
  mov rax, 170
  mov [rsp+64], rax
  mov rax, 162
  mov [r15+72], rax
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
  mov rax, [rsp+0]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+0]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+0], rax
  mov rax, 20
  mov [rsp+8], rax
  mov rbx, 9
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart6:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart6
  mov rax, 200
  mov [rsp+16], rax
  mov rax, 468
  mov [rsp+24], rax
  mov rax, -48
  mov [rsp+32], rax
  mov rax, 468
  mov [rsp+40], rax
  mov rax, -66
  mov [rsp+48], rax
  mov rax, -112
  mov [rsp+56], rax
  mov rax, 2000
  mov [rsp+64], rax
  mov rax, 170
  mov [rsp+72], rax
  mov rax, 162
  mov [r15+72], rax
  mov rax, [rsp+16]
  mov [r15+8], rax
  mov rax, [rsp+24]
  mov [r15+16], rax
  mov rax, [rsp+32]
  mov [r15+24], rax
  mov rax, [rsp+40]
  mov [r15+32], rax
  mov rax, [rsp+48]
  mov [r15+40], rax
  mov rax, [rsp+56]
  mov [r15+48], rax
  mov rax, [rsp+64]
  mov [r15+56], rax
  mov rax, [rsp+72]
  mov [r15+64], rax
  mov rax, [rsp+8]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+8]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+8], rax
  mov rax, [rsp+0]
  mov [rsp+16], rax
  mov rax, [rsp+8]
  mov [rsp+24], rax
  mov rbx, rax
  xor rbx, [rsp+16]
  and rbx, 3
  cmp rbx, 0
  jne throw_type_error
  cmp [rsp+16], rax
  mov rbx, 7
  mov rax, 3
  cmove rax, rbx
  jo throw_overflow_error
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  mov rax, [rsp+0]
  mov [rsp+16], rax
  mov rax, [rsp+8]
  mov [rsp+24], rax
  mov rbx, rax
  xor rbx, [rsp+16]
  and rbx, 3
  cmp rbx, 0
  jne throw_type_error
  mov rax, [rsp+24]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+16]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+24]
  mov rbx, [rsp+16]
  sub rax, 1
  sub rbx, 1
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop8
  mov rcx, rax
  imul rcx, 8
  mov rax, [rsp+16]
  mov rbx, [rsp+24]
  mov rdx, 7
  arr_loopstart7:
  mov rax, [rsp+16]
  mov rbx, [rsp+24]
  sub rax, 1
  sub rbx, 1
  add rax, rcx
  add rbx, rcx
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop8
  sub rcx, 8
  cmp rcx, 0
  jnz arr_loopstart7
  jz end_loop9
  break_loop8:
  mov rdx, 3
  end_loop9:
  mov rax, rdx
  mov rax, [rsp+24]
  mov rbx, [rsp+16]
  sub rsp, 24
  mov [rsp+0], rdi
  mov [rsp+8], rsi
  mov rdi, rax
  mov rsi, rbx
  call snek_eq
  mov rdi, [rsp+0]
  mov rsi, [rsp+8]
  add rsp, 24
  jo throw_overflow_error
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  mov rax, [rsp+0]
  mov rbx, rax
  and rbx, 3
  cmp rbx, 1
  jne throw_tag_check_error
  sub rax, 1
  mov r15, rax
  mov rax, 18
  mov [rsp+24], rax
  cmp rax, [r15+0]
  jg throw_bounds_error
  cmp rax, 0
  jl throw_bounds_error
  mov rax, 160
  mov rbx, [rsp+24]
  sar rbx, 1
  imul rbx, 8
  mov qword [r15+rbx], rax
  mov rax, r15
  add rax, 1
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  mov rax, [rsp+0]
  mov [rsp+16], rax
  mov rax, [rsp+8]
  mov [rsp+24], rax
  mov rbx, rax
  xor rbx, [rsp+16]
  and rbx, 3
  cmp rbx, 0
  jne throw_type_error
  mov rax, [rsp+24]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+16]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+24]
  mov rbx, [rsp+16]
  sub rax, 1
  sub rbx, 1
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop11
  mov rcx, rax
  imul rcx, 8
  mov rax, [rsp+16]
  mov rbx, [rsp+24]
  mov rdx, 7
  arr_loopstart10:
  mov rax, [rsp+16]
  mov rbx, [rsp+24]
  sub rax, 1
  sub rbx, 1
  add rax, rcx
  add rbx, rcx
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop11
  sub rcx, 8
  cmp rcx, 0
  jnz arr_loopstart10
  jz end_loop12
  break_loop11:
  mov rdx, 3
  end_loop12:
  mov rax, rdx
  mov rax, [rsp+24]
  mov rbx, [rsp+16]
  sub rsp, 24
  mov [rsp+0], rdi
  mov [rsp+8], rsi
  mov rdi, rax
  mov rsi, rbx
  call snek_eq
  mov rdi, [rsp+0]
  mov rsi, [rsp+8]
  add rsp, 24
  jo throw_overflow_error
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  mov rax, [rsp+0]
  mov rbx, rax
  and rbx, 3
  cmp rbx, 1
  jne throw_tag_check_error
  sub rax, 1
  mov r15, rax
  mov rax, 18
  mov [rsp+24], rax
  cmp rax, [r15+0]
  jg throw_bounds_error
  cmp rax, 0
  jl throw_bounds_error
  mov rax, 162
  mov rbx, [rsp+24]
  sar rbx, 1
  imul rbx, 8
  mov qword [r15+rbx], rax
  mov rax, r15
  add rax, 1
  sub rsp, 24
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 24
  mov rax, [rsp+0]
  mov [rsp+16], rax
  mov rax, [rsp+8]
  mov [rsp+24], rax
  mov rbx, rax
  xor rbx, [rsp+16]
  and rbx, 3
  cmp rbx, 0
  jne throw_type_error
  mov rax, [rsp+24]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+16]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+24]
  mov rbx, [rsp+16]
  sub rax, 1
  sub rbx, 1
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop14
  mov rcx, rax
  imul rcx, 8
  mov rax, [rsp+16]
  mov rbx, [rsp+24]
  mov rdx, 7
  arr_loopstart13:
  mov rax, [rsp+16]
  mov rbx, [rsp+24]
  sub rax, 1
  sub rbx, 1
  add rax, rcx
  add rbx, rcx
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop14
  sub rcx, 8
  cmp rcx, 0
  jnz arr_loopstart13
  jz end_loop15
  break_loop14:
  mov rdx, 3
  end_loop15:
  mov rax, rdx
  mov rax, [rsp+24]
  mov rbx, [rsp+16]
  sub rsp, 24
  mov [rsp+0], rdi
  mov [rsp+8], rsi
  mov rdi, rax
  mov rsi, rbx
  call snek_eq
  mov rdi, [rsp+0]
  mov rsi, [rsp+8]
  add rsp, 24
  jo throw_overflow_error
  sub rsp, 8
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 8
  mov rax, 12
  mov [rsp+0], rax
  mov rbx, 0
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart16:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart16
  mov rax, [rsp+0]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+0]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+0], rax
  mov rax, [rsp+0]
  mov [rsp+8], rax
  mov rax, [rsp+0]
  mov [rsp+16], rax
  mov rax, [rsp+8]
  mov [rsp+24], rax
  mov rbx, rax
  xor rbx, [rsp+16]
  and rbx, 3
  cmp rbx, 0
  jne throw_type_error
  cmp [rsp+16], rax
  mov rbx, 7
  mov rax, 3
  cmove rax, rbx
  jo throw_overflow_error
  sub rsp, 8
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 8
  mov rax, 20000
  mov [rsp+0], rax
  mov rbx, 0
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart17:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart17
  mov rax, [rsp+0]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+0]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+0], rax
  mov rax, 20002
  mov [rsp+8], rax
  mov rbx, 0
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart18:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart18
  mov rax, [rsp+8]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+8]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+8], rax
  mov rbx, rax
  xor rbx, [rsp+0]
  and rbx, 3
  cmp rbx, 0
  jne throw_type_error
  mov rax, [rsp+8]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+0]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+8]
  mov rbx, [rsp+0]
  sub rax, 1
  sub rbx, 1
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop20
  mov rcx, rax
  imul rcx, 8
  mov rax, [rsp+0]
  mov rbx, [rsp+8]
  mov rdx, 7
  arr_loopstart19:
  mov rax, [rsp+0]
  mov rbx, [rsp+8]
  sub rax, 1
  sub rbx, 1
  add rax, rcx
  add rbx, rcx
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop20
  sub rcx, 8
  cmp rcx, 0
  jnz arr_loopstart19
  jz end_loop21
  break_loop20:
  mov rdx, 3
  end_loop21:
  mov rax, rdx
  mov rax, [rsp+8]
  mov rbx, [rsp+0]
  sub rsp, 24
  mov [rsp+0], rdi
  mov [rsp+8], rsi
  mov rdi, rax
  mov rsi, rbx
  call snek_eq
  mov rdi, [rsp+0]
  mov rsi, [rsp+8]
  add rsp, 24
  jo throw_overflow_error
  sub rsp, 8
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 8
  mov rax, 20000
  mov [rsp+0], rax
  mov rbx, 0
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart22:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart22
  mov rax, [rsp+0]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+0]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+0], rax
  mov rax, 20000
  mov [rsp+8], rax
  mov rbx, 0
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart23:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart23
  mov rax, [rsp+8]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+8]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+8], rax
  mov rbx, rax
  xor rbx, [rsp+0]
  and rbx, 3
  cmp rbx, 0
  jne throw_type_error
  mov rax, [rsp+8]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+0]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+8]
  mov rbx, [rsp+0]
  sub rax, 1
  sub rbx, 1
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop25
  mov rcx, rax
  imul rcx, 8
  mov rax, [rsp+0]
  mov rbx, [rsp+8]
  mov rdx, 7
  arr_loopstart24:
  mov rax, [rsp+0]
  mov rbx, [rsp+8]
  sub rax, 1
  sub rbx, 1
  add rax, rcx
  add rbx, rcx
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop25
  sub rcx, 8
  cmp rcx, 0
  jnz arr_loopstart24
  jz end_loop26
  break_loop25:
  mov rdx, 3
  end_loop26:
  mov rax, rdx
  mov rax, [rsp+8]
  mov rbx, [rsp+0]
  sub rsp, 24
  mov [rsp+0], rdi
  mov [rsp+8], rsi
  mov rdi, rax
  mov rsi, rbx
  call snek_eq
  mov rdi, [rsp+0]
  mov rsi, [rsp+8]
  add rsp, 24
  jo throw_overflow_error
  sub rsp, 8
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 8
  mov rax, 10
  mov [rsp+0], rax
  mov rbx, 0
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart27:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart27
  mov rax, [rsp+0]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+0]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+0], rax
  mov rax, 10
  mov [rsp+8], rax
  mov rbx, 0
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart28:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart28
  mov rax, [rsp+8]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+8]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+8], rax
  mov rbx, rax
  xor rbx, [rsp+0]
  and rbx, 3
  cmp rbx, 0
  jne throw_type_error
  mov rax, [rsp+8]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+0]
  and rax, 3
  cmp rax, 1
  jne throw_type_error
  mov rax, [rsp+8]
  mov rbx, [rsp+0]
  sub rax, 1
  sub rbx, 1
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop30
  mov rcx, rax
  imul rcx, 8
  mov rax, [rsp+0]
  mov rbx, [rsp+8]
  mov rdx, 7
  arr_loopstart29:
  mov rax, [rsp+0]
  mov rbx, [rsp+8]
  sub rax, 1
  sub rbx, 1
  add rax, rcx
  add rbx, rcx
  mov rax, [rax+0]
  mov rbx, [rbx+0]
  cmp rax, rbx
  jne break_loop30
  sub rcx, 8
  cmp rcx, 0
  jnz arr_loopstart29
  jz end_loop31
  break_loop30:
  mov rdx, 3
  end_loop31:
  mov rax, rdx
  mov rax, [rsp+8]
  mov rbx, [rsp+0]
  sub rsp, 24
  mov [rsp+0], rdi
  mov [rsp+8], rsi
  mov rdi, rax
  mov rsi, rbx
  call snek_eq
  mov rdi, [rsp+0]
  mov rsi, [rsp+8]
  add rsp, 24
  jo throw_overflow_error
  sub rsp, 8
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 8
  mov rax, 10
  mov [rsp+0], rax
  mov rbx, 0
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart32:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart32
  mov rax, [rsp+0]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+0]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+0], rax
  mov rax, 10
  mov [rsp+8], rax
  mov rbx, 0
  imul rbx, 2
  cmp rbx, rax
  jg throw_too_many_elements_error
  imul rax, 8
  mov rbx, rax
  sar rbx, 1
  arr_loopstart33:
  mov qword [r15+rbx], 1
  sub rbx, 8
  jnz arr_loopstart33
  mov rax, [rsp+8]
  mov [r15+0], rax
  mov rax, r15
  add rax, 1
  mov rbx, [rsp+8]
  add rbx, 1
  imul rbx, 8
  add r15, rbx
  mov [rsp+8], rax
  mov rbx, rax
  xor rbx, [rsp+0]
  and rbx, 3
  cmp rbx, 0
  jne throw_type_error
  cmp [rsp+0], rax
  mov rbx, 7
  mov rax, 3
  cmove rax, rbx
  jo throw_overflow_error
  sub rsp, 8
  mov [rsp+0], rdi
  mov rdi, rax
  call snek_print
  mov rdi, [rsp+0]
  add rsp, 8
  add rsp, 112
  ret

[0m[1m[38;5;9mrint::rint:
[0m [0m[1m[38;5;12mvmovq  [0m rax, xmm0
[0m [0m[1m[38;5;12mmov    [0m cl, 52
[0m [0m[1m[38;5;12mbzhi   [0m rcx, rax, rcx
[0m [0m[1m[38;5;12mmovabs [0m rax, -2251799813685248
[0m [0m[1m[38;5;12madd    [0m rax, rcx
[0m [0m[1m[38;5;12mret[0m
[0m[0m[1m[38;5;10m__xmm@80000000000000008000000000000000:
[0m[0m[1m[38;5;10m__real@7ff8000000000000:
[0m[0m[1m[38;5;10m__real@7ff0000000000000:
[0m[0m[1m[38;5;10m_ZN4rint4main17h1aa9ac65c5338b87E:
[0m[0m[1m[38;5;10m.Lfunc_begin0:
[0m [0m[1m[38;5;12mpush   [0m rbp
[0m [0m[1m[38;5;12mpush   [0m r15
[0m [0m[1m[38;5;12mpush   [0m r14
[0m [0m[1m[38;5;12mpush   [0m r13
[0m [0m[1m[38;5;12mpush   [0m r12
[0m [0m[1m[38;5;12mpush   [0m rsi
[0m [0m[1m[38;5;12mpush   [0m rdi
[0m [0m[1m[38;5;12mpush   [0m rbx
[0m [0m[1m[38;5;12msub    [0m rsp, 1144
[0m [0m[1m[38;5;12mlea    [0m rbp, [rsp, +, 128]
[0m [0m[1m[38;5;12mvmovapd[0m xmmword, ptr, [rbp, +, 992], xmm9
[0m [0m[1m[38;5;12mvmovapd[0m xmmword, ptr, [rbp, +, 976], xmm8
[0m [0m[1m[38;5;12mvmovapd[0m xmmword, ptr, [rbp, +, 960], xmm7
[0m [0m[1m[38;5;12mvmovapd[0m xmmword, ptr, [rbp, +, 944], xmm6
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 936], -2
[0m [0m[1m[38;5;12mlea    [0m rcx, [rbp, +, 712]
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m std::env::args
[0m [0m[1m[38;5;12mvmovupd[0m ymm0, ymmword, ptr, [rbp, +, 712]
[0m [0m[1m[38;5;12mvmovupd[0m ymmword, ptr, [rbp, +, 816], ymm0
[0m [0m[1m[38;5;12mmov    [0m byte, ptr, [rbp, +, 935], 1
[0m [0m[1m[38;5;12mlea    [0m rcx, [rbp, +, 848]
[0m [0m[1m[38;5;12mlea    [0m rdx, [rbp, +, 816]
[0m [0m[1m[38;5;12mvzeroupper[0m
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m <std::env::Args as core::iter::traits::iterator::Iterator>::next
[0m [0m[1m[38;5;12mcmp    [0m qword, ptr, [rbp, +, 848], 0
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_22
[0m [0m[1m[38;5;12mmov    [0m rax, qword, ptr, [rbp, +, 864]
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 896], rax
[0m [0m[1m[38;5;12mvmovupd[0m xmm0, xmmword, ptr, [rbp, +, 848]
[0m [0m[1m[38;5;12mvmovapd[0m xmmword, ptr, [rbp, +, 880], xmm0
[0m [0m[1m[38;5;12mlea    [0m rcx, [rbp, -, 80]
[0m [0m[1m[38;5;12mlea    [0m rdx, [rbp, +, 816]
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m <std::env::Args as core::iter::traits::iterator::Iterator>::size_hint
[0m [0m[1m[38;5;12mmov    [0m rax, qword, ptr, [rbp, -, 80]
[0m [0m[1m[38;5;12minc    [0m rax
[0m [0m[1m[38;5;12mmov    [0m rcx, -1
[0m [0m[1m[38;5;12mcmovne [0m rcx, rax
[0m [0m[1m[38;5;12mcmp    [0m rcx, 5
[0m [0m[1m[38;5;12mmov    [0m ebx, 4
[0m [0m[1m[38;5;12mcmovae [0m rbx, rcx
[0m [0m[1m[38;5;12mmovabs [0m rax, 384307168202282326
[0m [0m[1m[38;5;12mxor    [0m esi, esi
[0m [0m[1m[38;5;12mcmp    [0m rbx, rax
[0m [0m[1m[38;5;12msetb   [0m al
[0m [0m[1m[38;5;12mjae    [0m[1m[38;5;10m .LBB12_133
[0m [0m[1m[38;5;12mlea    [0m rcx, [8*rbx]
[0m [0m[1m[38;5;12mmov    [0m sil, al
[0m [0m[1m[38;5;12mshl    [0m rsi, 3
[0m [0m[1m[38;5;12mlea    [0m r14, [rcx, +, 2*rcx]
[0m [0m[1m[38;5;12mtest   [0m r14, r14
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_29
[0m [0m[1m[38;5;12mmov    [0m rcx, r14
[0m [0m[1m[38;5;12mmov    [0m rdx, rsi
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m __rust_alloc
[0m [0m[1m[38;5;12mmov    [0m rdi, rax
[0m [0m[1m[38;5;12mtest   [0m rdi, rdi
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_30
[0m[0m[1m[38;5;10m.LBB12_6:
[0m [0m[1m[38;5;12mmov    [0m rax, qword, ptr, [rbp, +, 864]
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rdi, +, 16], rax
[0m [0m[1m[38;5;12mvmovups[0m xmm0, xmmword, ptr, [rbp, +, 848]
[0m [0m[1m[38;5;12mvmovups[0m xmmword, ptr, [rdi], xmm0
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 792], rdi
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 800], rbx
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 808], 1
[0m [0m[1m[38;5;12mvmovupd[0m ymm0, ymmword, ptr, [rbp, +, 816]
[0m [0m[1m[38;5;12mvmovupd[0m ymmword, ptr, [rbp, -, 80], ymm0
[0m [0m[1m[38;5;12mmov    [0m esi, 1
[0m [0m[1m[38;5;12mlea    [0m r12, [rbp, +, 744]
[0m [0m[1m[38;5;12mlea    [0m rbx, [rbp, -, 80]
[0m [0m[1m[38;5;12mlea    [0m r14, [rbp, +, 880]
[0m [0m[1m[38;5;12mmov    [0m r13, -1
[0m [0m[1m[38;5;12mlea    [0m r15, [rbp, +, 792]
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_9
[0m[0m[1m[38;5;10m.LBB12_7:
[0m [0m[1m[38;5;12mmov    [0m rdi, qword, ptr, [rbp, +, 792]
[0m[0m[1m[38;5;10m.LBB12_8:
[0m [0m[1m[38;5;12mlea    [0m rax, [rsi, +, 2*rsi]
[0m [0m[1m[38;5;12mmov    [0m rcx, qword, ptr, [rbp, +, 760]
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rdi, +, 8*rax, +, 16], rcx
[0m [0m[1m[38;5;12mvmovupd[0m xmm0, xmmword, ptr, [rbp, +, 744]
[0m [0m[1m[38;5;12mvmovupd[0m xmmword, ptr, [rdi, +, 8*rax], xmm0
[0m [0m[1m[38;5;12minc    [0m rsi
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 808], rsi
[0m[0m[1m[38;5;10m.LBB12_9:
[0m [0m[1m[38;5;12mmov    [0m rcx, r12
[0m [0m[1m[38;5;12mmov    [0m rdx, rbx
[0m [0m[1m[38;5;12mvzeroupper[0m
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m <std::env::Args as core::iter::traits::iterator::Iterator>::next
[0m [0m[1m[38;5;12mcmp    [0m qword, ptr, [rbp, +, 744], 0
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_14
[0m [0m[1m[38;5;12mmov    [0m rax, qword, ptr, [rbp, +, 760]
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 864], rax
[0m [0m[1m[38;5;12mvmovupd[0m xmm0, xmmword, ptr, [rbp, +, 744]
[0m [0m[1m[38;5;12mvmovapd[0m xmmword, ptr, [rbp, +, 848], xmm0
[0m [0m[1m[38;5;12mcmp    [0m rsi, qword, ptr, [rbp, +, 800]
[0m [0m[1m[38;5;12mjne    [0m[1m[38;5;10m .LBB12_8
[0m [0m[1m[38;5;12mmov    [0m rcx, r14
[0m [0m[1m[38;5;12mmov    [0m rdx, rbx
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m <std::env::Args as core::iter::traits::iterator::Iterator>::size_hint
[0m [0m[1m[38;5;12mmov    [0m r8, qword, ptr, [rbp, +, 880]
[0m [0m[1m[38;5;12minc    [0m r8
[0m [0m[1m[38;5;12mcmove  [0m r8, r13
[0m [0m[1m[38;5;12mmov    [0m rcx, r15
[0m [0m[1m[38;5;12mmov    [0m rdx, rsi
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_7
[0m[0m[1m[38;5;10m.LBB12_14:
[0m [0m[1m[38;5;12mmov    [0m rsi, qword, ptr, [rbp, -, 64]
[0m [0m[1m[38;5;12mmov    [0m rdi, qword, ptr, [rbp, -, 56]
[0m [0m[1m[38;5;12msub    [0m rdi, rsi
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_19
[0m [0m[1m[38;5;12mshr    [0m rdi, 5
[0m [0m[1m[38;5;12mshl    [0m rdi, 5
[0m [0m[1m[38;5;12mxor    [0m ebx, ebx
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_17
[0m[0m[1m[38;5;10m.LBB12_16:
[0m [0m[1m[38;5;12madd    [0m rbx, 32
[0m [0m[1m[38;5;12mcmp    [0m rdi, rbx
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_19
[0m[0m[1m[38;5;10m.LBB12_17:
[0m [0m[1m[38;5;12mmov    [0m rdx, qword, ptr, [rsi, +, rbx, +, 8]
[0m [0m[1m[38;5;12mtest   [0m rdx, rdx
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_16
[0m [0m[1m[38;5;12mmov    [0m rcx, qword, ptr, [rsi, +, rbx]
[0m [0m[1m[38;5;12mmov    [0m r8, rdx
[0m [0m[1m[38;5;12mnot    [0m r8
[0m [0m[1m[38;5;12mshr    [0m r8, 63
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m __rust_dealloc
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_16
[0m[0m[1m[38;5;10m.LBB12_19:
[0m [0m[1m[38;5;12mmov    [0m rdx, qword, ptr, [rbp, -, 72]
[0m [0m[1m[38;5;12mtest   [0m rdx, rdx
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_21
[0m [0m[1m[38;5;12mmov    [0m rcx, qword, ptr, [rbp, -, 80]
[0m [0m[1m[38;5;12mshl    [0m rdx, 5
[0m [0m[1m[38;5;12mmov    [0m r8d, 8
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m __rust_dealloc
[0m[0m[1m[38;5;10m.LBB12_21:
[0m [0m[1m[38;5;12mvmovupd[0m xmm0, xmmword, ptr, [rbp, +, 792]
[0m [0m[1m[38;5;12mvmovapd[0m xmmword, ptr, [rbp, +, 880], xmm0
[0m [0m[1m[38;5;12mmov    [0m rax, qword, ptr, [rbp, +, 808]
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 896], rax
[0m [0m[1m[38;5;12mmov    [0m rsi, qword, ptr, [rbp, +, 880]
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_33
[0m[0m[1m[38;5;10m.LBB12_22:
[0m [0m[1m[38;5;12mmov    [0m rsi, qword, ptr, [rbp, +, 832]
[0m [0m[1m[38;5;12mmov    [0m rdi, qword, ptr, [rbp, +, 840]
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 880], 8
[0m [0m[1m[38;5;12mvxorpd [0m xmm0, xmm0, xmm0
[0m [0m[1m[38;5;12mvmovupd[0m xmmword, ptr, [rbp, +, 888], xmm0
[0m [0m[1m[38;5;12msub    [0m rdi, rsi
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_27
[0m [0m[1m[38;5;12mshr    [0m rdi, 5
[0m [0m[1m[38;5;12mshl    [0m rdi, 5
[0m [0m[1m[38;5;12mxor    [0m ebx, ebx
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_25
[0m[0m[1m[38;5;10m.LBB12_24:
[0m [0m[1m[38;5;12madd    [0m rbx, 32
[0m [0m[1m[38;5;12mcmp    [0m rdi, rbx
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_27
[0m[0m[1m[38;5;10m.LBB12_25:
[0m [0m[1m[38;5;12mmov    [0m rdx, qword, ptr, [rsi, +, rbx, +, 8]
[0m [0m[1m[38;5;12mtest   [0m rdx, rdx
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_24
[0m [0m[1m[38;5;12mmov    [0m rcx, qword, ptr, [rsi, +, rbx]
[0m [0m[1m[38;5;12mmov    [0m r8, rdx
[0m [0m[1m[38;5;12mnot    [0m r8
[0m [0m[1m[38;5;12mshr    [0m r8, 63
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m __rust_dealloc
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_24
[0m[0m[1m[38;5;10m.LBB12_27:
[0m [0m[1m[38;5;12mmov    [0m rdx, qword, ptr, [rbp, +, 824]
[0m [0m[1m[38;5;12mtest   [0m rdx, rdx
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_31
[0m [0m[1m[38;5;12mmov    [0m rcx, qword, ptr, [rbp, +, 816]
[0m [0m[1m[38;5;12mshl    [0m rdx, 5
[0m [0m[1m[38;5;12mmov    [0m esi, 8
[0m [0m[1m[38;5;12mmov    [0m r8d, 8
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m __rust_dealloc
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_32
[0m[0m[1m[38;5;10m.LBB12_29:
[0m [0m[1m[38;5;12mmov    [0m rdi, rsi
[0m [0m[1m[38;5;12mtest   [0m rdi, rdi
[0m [0m[1m[38;5;12mjne    [0m[1m[38;5;10m .LBB12_6
[0m[0m[1m[38;5;10m.LBB12_30:
[0m [0m[1m[38;5;12mmov    [0m rcx, r14
[0m [0m[1m[38;5;12mmov    [0m rdx, rsi
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m alloc::alloc::handle_alloc_error
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_134
[0m[0m[1m[38;5;10m.LBB12_31:
[0m [0m[1m[38;5;12mmov    [0m esi, 8
[0m[0m[1m[38;5;10m.LBB12_32:
[0m [0m[1m[38;5;12mxor    [0m eax, eax
[0m[0m[1m[38;5;10m.LBB12_33:
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 776], rax
[0m [0m[1m[38;5;12mlea    [0m rax, [rax, +, 2*rax]
[0m [0m[1m[38;5;12mlea    [0m rax, [rsi, +, 8*rax]
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 784], rax
[0m [0m[1m[38;5;12mmov    [0m eax, 1
[0m [0m[1m[38;5;12mxor    [0m ecx, ecx
[0m [0m[1m[38;5;12mvmovsd [0m xmm8, qword, ptr, [rip, +, __real@7ff8000000000000]
[0m [0m[1m[38;5;12mvmovapd[0m xmm7, xmmword, ptr, [rip, +, __xmm@80000000000000008000000000000000]
[0m [0m[1m[38;5;12mvmovsd [0m xmm9, qword, ptr, [rip, +, __real@7ff0000000000000]
[0m [0m[1m[38;5;12mlea    [0m r12, [rip, +, __unnamed_2]
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 768], rsi
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 904], rsi
[0m [0m[1m[38;5;12mtest   [0m cl, 1
[0m [0m[1m[38;5;12mjne    [0m[1m[38;5;10m .LBB12_34
[0m[0m[1m[38;5;10m.LBB12_35:
[0m [0m[1m[38;5;12mmov    [0m rdx, qword, ptr, [rbp, +, 784]
[0m [0m[1m[38;5;12mmov    [0m rbx, qword, ptr, [rbp, +, 904]
[0m [0m[1m[38;5;12msub    [0m rdx, rbx
[0m [0m[1m[38;5;12mmovabs [0m rcx, -6148914691236517205
[0m [0m[1m[38;5;12mmulx   [0m rcx, rcx, rcx
[0m [0m[1m[38;5;12mshr    [0m rcx, 4
[0m [0m[1m[38;5;12mcmp    [0m rcx, rax
[0m [0m[1m[38;5;12mjbe    [0m[1m[38;5;10m .LBB12_119
[0m [0m[1m[38;5;12mlea    [0m rax, [rax, +, 2*rax]
[0m [0m[1m[38;5;12mlea    [0m rbx, [rbx, +, 8*rax]
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_37
[0m[0m[1m[38;5;10m.LBB12_34:
[0m [0m[1m[38;5;12mmov    [0m rbx, qword, ptr, [rbp, +, 904]
[0m [0m[1m[38;5;12mcmp    [0m rbx, qword, ptr, [rbp, +, 784]
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_119
[0m[0m[1m[38;5;10m.LBB12_37:
[0m [0m[1m[38;5;12mmov    [0m r15, qword, ptr, [rbx, +, 16]
[0m [0m[1m[38;5;12mtest   [0m r15, r15
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_130
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 904], rbx
[0m [0m[1m[38;5;12mmov    [0m r14, qword, ptr, [rbx]
[0m [0m[1m[38;5;12mmovzx  [0m esi, byte, ptr, [r14]
[0m [0m[1m[38;5;12mcmp    [0m esi, 43
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_40
[0m [0m[1m[38;5;12mcmp    [0m esi, 45
[0m [0m[1m[38;5;12mjne    [0m[1m[38;5;10m .LBB12_42
[0m[0m[1m[38;5;10m.LBB12_40:
[0m [0m[1m[38;5;12mdec    [0m r15
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_131
[0m [0m[1m[38;5;12minc    [0m r14
[0m[0m[1m[38;5;10m.LBB12_42:
[0m [0m[1m[38;5;12mcmp    [0m sil, 45
[0m [0m[1m[38;5;12msete   [0m r9b
[0m [0m[1m[38;5;12mlea    [0m rcx, [rbp, -, 80]
[0m [0m[1m[38;5;12mmov    [0m rdx, r14
[0m [0m[1m[38;5;12mmov    [0m r8, r15
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::parse::parse_number
[0m [0m[1m[38;5;12mmovzx  [0m eax, byte, ptr, [rbp, -, 64]
[0m [0m[1m[38;5;12mmov    [0m byte, ptr, [rbp, +, 934], al
[0m [0m[1m[38;5;12mcmp    [0m al, 2
[0m [0m[1m[38;5;12mjne    [0m[1m[38;5;10m .LBB12_56
[0m [0m[1m[38;5;12mcmp    [0m r15, 3
[0m [0m[1m[38;5;12mjb     [0m[1m[38;5;10m .LBB12_129
[0m [0m[1m[38;5;12mmov    [0m eax, 3
[0m [0m[1m[38;5;12mcmovb  [0m rax, r15
[0m [0m[1m[38;5;12mmovzx  [0m ecx, byte, ptr, [r14]
[0m [0m[1m[38;5;12mxor    [0m cl, 110
[0m [0m[1m[38;5;12mcmp    [0m rax, 1
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_48
[0m [0m[1m[38;5;12mmovzx  [0m edx, byte, ptr, [r14, +, 1]
[0m [0m[1m[38;5;12mxor    [0m dl, 97
[0m [0m[1m[38;5;12mor     [0m cl, dl
[0m [0m[1m[38;5;12mcmp    [0m rax, 2
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_48
[0m [0m[1m[38;5;12mmovzx  [0m edx, byte, ptr, [r14, +, 2]
[0m [0m[1m[38;5;12mxor    [0m dl, 110
[0m [0m[1m[38;5;12mor     [0m cl, dl
[0m[0m[1m[38;5;10m.LBB12_48:
[0m [0m[1m[38;5;12mtest   [0m cl, -33
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_63
[0m [0m[1m[38;5;12mlea    [0m rcx, [rax, -, 1]
[0m [0m[1m[38;5;12mcmp    [0m rcx, 7
[0m [0m[1m[38;5;12mjae    [0m[1m[38;5;10m .LBB12_127
[0m [0m[1m[38;5;12mtest   [0m rax, rax
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_53
[0m [0m[1m[38;5;12mxor    [0m ecx, ecx
[0m [0m[1m[38;5;12mxor    [0m edx, edx
[0m[0m[1m[38;5;10m.LBB12_52:
[0m [0m[1m[38;5;12mmovzx  [0m ebx, byte, ptr, [rdx, +, r12]
[0m [0m[1m[38;5;12mxor    [0m bl, byte, ptr, [r14, +, rdx]
[0m [0m[1m[38;5;12mor     [0m cl, bl
[0m [0m[1m[38;5;12minc    [0m rdx
[0m [0m[1m[38;5;12mcmp    [0m rax, rdx
[0m [0m[1m[38;5;12mjne    [0m[1m[38;5;10m .LBB12_52
[0m[0m[1m[38;5;10m.LBB12_53:
[0m [0m[1m[38;5;12mtest   [0m cl, -33
[0m [0m[1m[38;5;12mjne    [0m[1m[38;5;10m .LBB12_129
[0m [0m[1m[38;5;12mmov    [0m rcx, r14
[0m [0m[1m[38;5;12mmov    [0m rdx, r15
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::parse::parse_partial_inf_nan::parse_inf_rest
[0m [0m[1m[38;5;12mvmovapd[0m xmm0, xmm9
[0m [0m[1m[38;5;12mcmp    [0m rax, r15
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_64
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_129
[0m[0m[1m[38;5;10m.LBB12_56:
[0m [0m[1m[38;5;12mmov    [0m rdi, qword, ptr, [rbp, -, 80]
[0m [0m[1m[38;5;12mmov    [0m rbx, qword, ptr, [rbp, -, 72]
[0m [0m[1m[38;5;12mmovzx  [0m eax, byte, ptr, [rbp, -, 63]
[0m [0m[1m[38;5;12mmov    [0m byte, ptr, [rbp, +, 920], al
[0m [0m[1m[38;5;12mlea    [0m rax, [rdi, +, 22]
[0m [0m[1m[38;5;12mcmp    [0m rax, 59
[0m [0m[1m[38;5;12mja     [0m[1m[38;5;10m .LBB12_70
[0m [0m[1m[38;5;12mmovabs [0m rax, 9007199254740992
[0m [0m[1m[38;5;12mcmp    [0m rbx, rax
[0m [0m[1m[38;5;12mja     [0m[1m[38;5;10m .LBB12_70
[0m [0m[1m[38;5;12mcmp    [0m byte, ptr, [rbp, +, 920], 0
[0m [0m[1m[38;5;12mjne    [0m[1m[38;5;10m .LBB12_70
[0m [0m[1m[38;5;12mcmp    [0m rdi, 22
[0m [0m[1m[38;5;12mjg     [0m[1m[38;5;10m .LBB12_65
[0m [0m[1m[38;5;12mmov    [0m rcx, rbx
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m <f64 as core::num::dec2flt::float::RawFloat>::from_u64
[0m [0m[1m[38;5;12mvmovapd[0m xmm6, xmm0
[0m [0m[1m[38;5;12mtest   [0m rdi, rdi
[0m [0m[1m[38;5;12mjs     [0m[1m[38;5;10m .LBB12_83
[0m [0m[1m[38;5;12mmov    [0m rcx, rdi
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m <f64 as core::num::dec2flt::float::RawFloat>::pow10_fast_path
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_69
[0m[0m[1m[38;5;10m.LBB12_63:
[0m [0m[1m[38;5;12mmov    [0m eax, 3
[0m [0m[1m[38;5;12mvmovapd[0m xmm0, xmm8
[0m [0m[1m[38;5;12mcmp    [0m rax, r15
[0m [0m[1m[38;5;12mjne    [0m[1m[38;5;10m .LBB12_129
[0m[0m[1m[38;5;10m.LBB12_64:
[0m [0m[1m[38;5;12mcmp    [0m sil, 45
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_116
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_117
[0m[0m[1m[38;5;10m.LBB12_65:
[0m [0m[1m[38;5;12mmov    [0m rax, rbx
[0m [0m[1m[38;5;12mlea    [0m rcx, [rip, +, __unnamed_3]
[0m [0m[1m[38;5;12mmul    [0m qword, ptr, [rcx, +, 8*rdi, -, 176]
[0m [0m[1m[38;5;12mjo     [0m[1m[38;5;10m .LBB12_70
[0m [0m[1m[38;5;12mmovabs [0m rcx, 9007199254740992
[0m [0m[1m[38;5;12mcmp    [0m rax, rcx
[0m [0m[1m[38;5;12mja     [0m[1m[38;5;10m .LBB12_70
[0m [0m[1m[38;5;12mmov    [0m rcx, rax
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m <f64 as core::num::dec2flt::float::RawFloat>::from_u64
[0m [0m[1m[38;5;12mvmovapd[0m xmm6, xmm0
[0m [0m[1m[38;5;12mmov    [0m ecx, 22
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m <f64 as core::num::dec2flt::float::RawFloat>::pow10_fast_path
[0m[0m[1m[38;5;10m.LBB12_69:
[0m [0m[1m[38;5;12mvmulsd [0m xmm0, xmm6, xmm0
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_115
[0m[0m[1m[38;5;10m.LBB12_70:
[0m [0m[1m[38;5;12mmov    [0m rcx, rdi
[0m [0m[1m[38;5;12mmov    [0m rdx, rbx
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::lemire::compute_float
[0m [0m[1m[38;5;12mmov    [0m r13, rax
[0m [0m[1m[38;5;12mmov    [0m esi, edx
[0m [0m[1m[38;5;12mcmp    [0m byte, ptr, [rbp, +, 920], 0
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_75
[0m [0m[1m[38;5;12mtest   [0m esi, esi
[0m [0m[1m[38;5;12mjs     [0m[1m[38;5;10m .LBB12_75
[0m [0m[1m[38;5;12minc    [0m rbx
[0m [0m[1m[38;5;12mmov    [0m rcx, rdi
[0m [0m[1m[38;5;12mmov    [0m rdx, rbx
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::lemire::compute_float
[0m [0m[1m[38;5;12mcmp    [0m r13, rax
[0m [0m[1m[38;5;12msetne  [0m al
[0m [0m[1m[38;5;12mmov    [0m rcx, rsi
[0m [0m[1m[38;5;12mcmp    [0m ecx, edx
[0m [0m[1m[38;5;12msetne  [0m dl
[0m [0m[1m[38;5;12mor     [0m dl, al
[0m [0m[1m[38;5;12mjne    [0m[1m[38;5;10m .LBB12_76
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_114
[0m[0m[1m[38;5;10m.LBB12_75:
[0m [0m[1m[38;5;12mmov    [0m rcx, rsi
[0m [0m[1m[38;5;12mtest   [0m ecx, ecx
[0m [0m[1m[38;5;12mjns    [0m[1m[38;5;10m .LBB12_114
[0m[0m[1m[38;5;10m.LBB12_76:
[0m [0m[1m[38;5;12mxor    [0m ecx, ecx
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::common::BiasedFp::zero_pow2
[0m [0m[1m[38;5;12mmov    [0m r13, rax
[0m [0m[1m[38;5;12mmov    [0m edi, edx
[0m [0m[1m[38;5;12mmov    [0m ecx, 2047
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::common::BiasedFp::zero_pow2
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 920], rax
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 912], rdx
[0m [0m[1m[38;5;12mlea    [0m rcx, [rbp, -, 80]
[0m [0m[1m[38;5;12mmov    [0m rdx, r14
[0m [0m[1m[38;5;12mmov    [0m r8, r15
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::decimal::parse_decimal
[0m [0m[1m[38;5;12mcmp    [0m qword, ptr, [rbp, -, 80], 0
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_113
[0m [0m[1m[38;5;12mmov    [0m eax, dword, ptr, [rbp, -, 72]
[0m [0m[1m[38;5;12mcmp    [0m eax, -324
[0m [0m[1m[38;5;12mjl     [0m[1m[38;5;10m .LBB12_113
[0m [0m[1m[38;5;12mcmp    [0m eax, 309
[0m [0m[1m[38;5;12mjle    [0m[1m[38;5;10m .LBB12_87
[0m[0m[1m[38;5;10m.LBB12_82:
[0m [0m[1m[38;5;12mmov    [0m r13, qword, ptr, [rbp, +, 920]
[0m [0m[1m[38;5;12mmov    [0m rcx, qword, ptr, [rbp, +, 912]
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_114
[0m[0m[1m[38;5;10m.LBB12_83:
[0m [0m[1m[38;5;12mneg    [0m rdi
[0m [0m[1m[38;5;12mmov    [0m rcx, rdi
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m <f64 as core::num::dec2flt::float::RawFloat>::pow10_fast_path
[0m [0m[1m[38;5;12mvdivsd [0m xmm0, xmm6, xmm0
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_115
[0m[0m[1m[38;5;10m.LBB12_87:
[0m [0m[1m[38;5;12mxor    [0m esi, esi
[0m [0m[1m[38;5;12mtest   [0m eax, eax
[0m [0m[1m[38;5;12mjle    [0m[1m[38;5;10m .LBB12_93
[0m[0m[1m[38;5;10m.LBB12_88:
[0m [0m[1m[38;5;12mmov    [0m ebx, 60
[0m [0m[1m[38;5;12mcmp    [0m eax, 18
[0m [0m[1m[38;5;12mja     [0m[1m[38;5;10m .LBB12_90
[0m [0m[1m[38;5;12mmov    [0m eax, eax
[0m [0m[1m[38;5;12mlea    [0m rcx, [rip, +, __unnamed_4]
[0m [0m[1m[38;5;12mmovzx  [0m ebx, byte, ptr, [rax, +, rcx]
[0m[0m[1m[38;5;10m.LBB12_90:
[0m [0m[1m[38;5;12mlea    [0m rcx, [rbp, -, 80]
[0m [0m[1m[38;5;12mmov    [0m rdx, rbx
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::decimal::Decimal::right_shift
[0m [0m[1m[38;5;12mmov    [0m eax, dword, ptr, [rbp, -, 72]
[0m [0m[1m[38;5;12mcmp    [0m eax, -2047
[0m [0m[1m[38;5;12mjl     [0m[1m[38;5;10m .LBB12_113
[0m [0m[1m[38;5;12madd    [0m esi, ebx
[0m [0m[1m[38;5;12mtest   [0m eax, eax
[0m [0m[1m[38;5;12mjg     [0m[1m[38;5;10m .LBB12_88
[0m[0m[1m[38;5;10m.LBB12_93:
[0m [0m[1m[38;5;12mtest   [0m eax, eax
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_96
[0m [0m[1m[38;5;12mneg    [0m eax
[0m [0m[1m[38;5;12mmov    [0m edi, 60
[0m [0m[1m[38;5;12mcmp    [0m eax, 18
[0m [0m[1m[38;5;12mja     [0m[1m[38;5;10m .LBB12_98
[0m [0m[1m[38;5;12mmov    [0m eax, eax
[0m [0m[1m[38;5;12mlea    [0m rcx, [rip, +, __unnamed_4]
[0m [0m[1m[38;5;12mmovzx  [0m edi, byte, ptr, [rax, +, rcx]
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_98
[0m[0m[1m[38;5;10m.LBB12_96:
[0m [0m[1m[38;5;12mmovzx  [0m eax, byte, ptr, [rbp, -, 67]
[0m [0m[1m[38;5;12mcmp    [0m al, 4
[0m [0m[1m[38;5;12mja     [0m[1m[38;5;10m .LBB12_101
[0m [0m[1m[38;5;12mcmp    [0m al, 2
[0m [0m[1m[38;5;12mmov    [0m edi, 0
[0m [0m[1m[38;5;12madc    [0m rdi, 1
[0m[0m[1m[38;5;10m.LBB12_98:
[0m [0m[1m[38;5;12mlea    [0m rcx, [rbp, -, 80]
[0m [0m[1m[38;5;12mmov    [0m rdx, rdi
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::decimal::Decimal::left_shift
[0m [0m[1m[38;5;12mmov    [0m eax, dword, ptr, [rbp, -, 72]
[0m [0m[1m[38;5;12mcmp    [0m eax, 2047
[0m [0m[1m[38;5;12mjg     [0m[1m[38;5;10m .LBB12_82
[0m [0m[1m[38;5;12msub    [0m esi, edi
[0m [0m[1m[38;5;12mtest   [0m eax, eax
[0m [0m[1m[38;5;12mjle    [0m[1m[38;5;10m .LBB12_93
[0m[0m[1m[38;5;10m.LBB12_101:
[0m [0m[1m[38;5;12mdec    [0m esi
[0m [0m[1m[38;5;12mcmp    [0m esi, -1023
[0m [0m[1m[38;5;12mjg     [0m[1m[38;5;10m .LBB12_104
[0m[0m[1m[38;5;10m.LBB12_102:
[0m [0m[1m[38;5;12mmov    [0m edi, -1022
[0m [0m[1m[38;5;12msub    [0m edi, esi
[0m [0m[1m[38;5;12mcmp    [0m edi, 60
[0m [0m[1m[38;5;12mmov    [0m eax, 60
[0m [0m[1m[38;5;12mcmovae [0m edi, eax
[0m [0m[1m[38;5;12mlea    [0m rcx, [rbp, -, 80]
[0m [0m[1m[38;5;12mmov    [0m rdx, rdi
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::decimal::Decimal::right_shift
[0m [0m[1m[38;5;12madd    [0m edi, esi
[0m [0m[1m[38;5;12mmov    [0m esi, edi
[0m [0m[1m[38;5;12mcmp    [0m edi, -1022
[0m [0m[1m[38;5;12mjb     [0m[1m[38;5;10m .LBB12_102
[0m[0m[1m[38;5;10m.LBB12_104:
[0m [0m[1m[38;5;12mlea    [0m eax, [rsi, +, 1023]
[0m [0m[1m[38;5;12mcmp    [0m eax, 2046
[0m [0m[1m[38;5;12mjg     [0m[1m[38;5;10m .LBB12_82
[0m [0m[1m[38;5;12mmov    [0m edx, 53
[0m [0m[1m[38;5;12mlea    [0m rcx, [rbp, -, 80]
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::decimal::Decimal::left_shift
[0m [0m[1m[38;5;12mlea    [0m rcx, [rbp, -, 80]
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::decimal::Decimal::round
[0m [0m[1m[38;5;12mmov    [0m rcx, rax
[0m [0m[1m[38;5;12mshr    [0m rcx, 53
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_112
[0m [0m[1m[38;5;12mmov    [0m edx, 1
[0m [0m[1m[38;5;12mlea    [0m rcx, [rbp, -, 80]
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::decimal::Decimal::right_shift
[0m [0m[1m[38;5;12mlea    [0m rcx, [rbp, -, 80]
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::decimal::Decimal::round
[0m [0m[1m[38;5;12mlea    [0m ecx, [rsi, +, 1024]
[0m [0m[1m[38;5;12mcmp    [0m ecx, 2046
[0m [0m[1m[38;5;12mmov    [0m r13, qword, ptr, [rbp, +, 920]
[0m [0m[1m[38;5;12mmov    [0m rcx, qword, ptr, [rbp, +, 912]
[0m [0m[1m[38;5;12mjg     [0m[1m[38;5;10m .LBB12_114
[0m [0m[1m[38;5;12minc    [0m esi
[0m[0m[1m[38;5;10m.LBB12_112:
[0m [0m[1m[38;5;12mmov    [0m cl, 52
[0m [0m[1m[38;5;12mbzhi   [0m r13, rax, rcx
[0m [0m[1m[38;5;12mshr    [0m rax, 52
[0m [0m[1m[38;5;12mcmp    [0m rax, 1
[0m [0m[1m[38;5;12msbb    [0m esi, 0
[0m [0m[1m[38;5;12madd    [0m esi, 1023
[0m [0m[1m[38;5;12mmov    [0m ecx, esi
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_114
[0m[0m[1m[38;5;10m.LBB12_113:
[0m [0m[1m[38;5;12mmov    [0m ecx, edi
[0m[0m[1m[38;5;10m.LBB12_114:
[0m [0m[1m[38;5;12mshl    [0m rcx, 52
[0m [0m[1m[38;5;12mor     [0m rcx, r13
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m <f64 as core::num::dec2flt::float::RawFloat>::from_u64_bits
[0m[0m[1m[38;5;10m.LBB12_115:
[0m [0m[1m[38;5;12mcmp    [0m byte, ptr, [rbp, +, 934], 0
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_117
[0m[0m[1m[38;5;10m.LBB12_116:
[0m [0m[1m[38;5;12mvxorpd [0m xmm0, xmm0, xmm7
[0m[0m[1m[38;5;10m.LBB12_117:
[0m [0m[1m[38;5;12mvmovsd [0m qword, ptr, [rbp, +, 848], xmm0
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m rint::rint
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 712], rax
[0m [0m[1m[38;5;12mlea    [0m rax, [rbp, +, 848]
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 816], rax
[0m [0m[1m[38;5;12mlea    [0m rax, [rip, +, _ZN4core3fmt5float52_$LT$impl$u20$core..fmt..Display$u20$for$u20$f64$GT$3fmt17hc213f8cd4ed81722E]
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 824], rax
[0m [0m[1m[38;5;12mlea    [0m rax, [rbp, +, 712]
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 832], rax
[0m [0m[1m[38;5;12mlea    [0m rax, [rip, +, _ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17h7b7441124c72c24eE]
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, +, 840], rax
[0m [0m[1m[38;5;12mlea    [0m rax, [rip, +, __unnamed_5]
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, -, 80], rax
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, -, 72], 3
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, -, 64], 0
[0m [0m[1m[38;5;12mlea    [0m rax, [rbp, +, 816]
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, -, 48], rax
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rbp, -, 40], 2
[0m [0m[1m[38;5;12mlea    [0m rcx, [rbp, -, 80]
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m std::io::stdio::_print
[0m [0m[1m[38;5;12madd    [0m qword, ptr, [rbp, +, 904], 24
[0m [0m[1m[38;5;12mmov    [0m cl, 1
[0m [0m[1m[38;5;12mxor    [0m eax, eax
[0m [0m[1m[38;5;12mtest   [0m cl, 1
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_35
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_34
[0m[0m[1m[38;5;10m.LBB12_119:
[0m [0m[1m[38;5;12mmov    [0m rax, qword, ptr, [rbp, +, 776]
[0m [0m[1m[38;5;12mtest   [0m rax, rax
[0m [0m[1m[38;5;12mmov    [0m rbx, qword, ptr, [rbp, +, 768]
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_124
[0m [0m[1m[38;5;12mshl    [0m rax, 3
[0m [0m[1m[38;5;12mlea    [0m rsi, [rax, +, 2*rax]
[0m [0m[1m[38;5;12mxor    [0m edi, edi
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_122
[0m[0m[1m[38;5;10m.LBB12_121:
[0m [0m[1m[38;5;12madd    [0m rdi, 24
[0m [0m[1m[38;5;12mcmp    [0m rsi, rdi
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_124
[0m[0m[1m[38;5;10m.LBB12_122:
[0m [0m[1m[38;5;12mmov    [0m rdx, qword, ptr, [rbx, +, rdi, +, 8]
[0m [0m[1m[38;5;12mtest   [0m rdx, rdx
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_121
[0m [0m[1m[38;5;12mmov    [0m rcx, qword, ptr, [rbx, +, rdi]
[0m [0m[1m[38;5;12mmov    [0m r8, rdx
[0m [0m[1m[38;5;12mnot    [0m r8
[0m [0m[1m[38;5;12mshr    [0m r8, 63
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m __rust_dealloc
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_121
[0m[0m[1m[38;5;10m.LBB12_124:
[0m [0m[1m[38;5;12mmov    [0m rax, qword, ptr, [rbp, +, 888]
[0m [0m[1m[38;5;12mtest   [0m rax, rax
[0m [0m[1m[38;5;12mje     [0m[1m[38;5;10m .LBB12_126
[0m [0m[1m[38;5;12mshl    [0m rax, 3
[0m [0m[1m[38;5;12mlea    [0m rdx, [rax, +, 2*rax]
[0m [0m[1m[38;5;12mmov    [0m r8d, 8
[0m [0m[1m[38;5;12mmov    [0m rcx, rbx
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m __rust_dealloc
[0m[0m[1m[38;5;10m.LBB12_126:
[0m [0m[1m[38;5;12mvmovaps[0m xmm6, xmmword, ptr, [rbp, +, 944]
[0m [0m[1m[38;5;12mvmovaps[0m xmm7, xmmword, ptr, [rbp, +, 960]
[0m [0m[1m[38;5;12mvmovaps[0m xmm8, xmmword, ptr, [rbp, +, 976]
[0m [0m[1m[38;5;12mvmovaps[0m xmm9, xmmword, ptr, [rbp, +, 992]
[0m [0m[1m[38;5;12madd    [0m rsp, 1144
[0m [0m[1m[38;5;12mpop    [0m rbx
[0m [0m[1m[38;5;12mpop    [0m rdi
[0m [0m[1m[38;5;12mpop    [0m rsi
[0m [0m[1m[38;5;12mpop    [0m r12
[0m [0m[1m[38;5;12mpop    [0m r13
[0m [0m[1m[38;5;12mpop    [0m r14
[0m [0m[1m[38;5;12mpop    [0m r15
[0m [0m[1m[38;5;12mpop    [0m rbp
[0m [0m[1m[38;5;12mret[0m
[0m[0m[1m[38;5;10m.LBB12_127:
[0m [0m[1m[38;5;12mxor    [0m edi, edi
[0m [0m[1m[38;5;12mxor    [0m eax, eax
[0m[0m[1m[38;5;10m.LBB12_128:
[0m [0m[1m[38;5;12mmovzx  [0m edx, byte, ptr, [rax, +, r12]
[0m [0m[1m[38;5;12mxor    [0m dl, byte, ptr, [r14, +, rax]
[0m [0m[1m[38;5;12mmovzx  [0m r8d, byte, ptr, [rax, +, r12, +, 1]
[0m [0m[1m[38;5;12mxor    [0m r8b, byte, ptr, [r14, +, rax, +, 1]
[0m [0m[1m[38;5;12mmovzx  [0m ebx, byte, ptr, [rax, +, r12, +, 2]
[0m [0m[1m[38;5;12mxor    [0m bl, byte, ptr, [r14, +, rax, +, 2]
[0m [0m[1m[38;5;12mor     [0m dl, dil
[0m [0m[1m[38;5;12mor     [0m bl, r8b
[0m [0m[1m[38;5;12mor     [0m bl, dl
[0m [0m[1m[38;5;12mmovzx  [0m r8d, byte, ptr, [rax, +, r12, +, 3]
[0m [0m[1m[38;5;12mxor    [0m r8b, byte, ptr, [r14, +, rax, +, 3]
[0m [0m[1m[38;5;12mmovzx  [0m edx, byte, ptr, [rax, +, r12, +, 4]
[0m [0m[1m[38;5;12mxor    [0m dl, byte, ptr, [r14, +, rax, +, 4]
[0m [0m[1m[38;5;12mmovzx  [0m ecx, byte, ptr, [rax, +, r12, +, 5]
[0m [0m[1m[38;5;12mxor    [0m cl, byte, ptr, [r14, +, rax, +, 5]
[0m [0m[1m[38;5;12mor     [0m dl, r8b
[0m [0m[1m[38;5;12mor     [0m cl, dl
[0m [0m[1m[38;5;12mor     [0m cl, bl
[0m [0m[1m[38;5;12mmovzx  [0m edx, byte, ptr, [rax, +, r12, +, 6]
[0m [0m[1m[38;5;12mxor    [0m dl, byte, ptr, [r14, +, rax, +, 6]
[0m [0m[1m[38;5;12mmovzx  [0m edi, byte, ptr, [rax, +, r12, +, 7]
[0m [0m[1m[38;5;12mxor    [0m dil, byte, ptr, [r14, +, rax, +, 7]
[0m [0m[1m[38;5;12madd    [0m rax, 8
[0m [0m[1m[38;5;12mor     [0m dil, dl
[0m [0m[1m[38;5;12mor     [0m dil, cl
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_128
[0m[0m[1m[38;5;10m.LBB12_129:
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::pfe_invalid
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_132
[0m[0m[1m[38;5;10m.LBB12_130:
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::pfe_empty
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_132
[0m[0m[1m[38;5;10m.LBB12_131:
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::num::dec2flt::pfe_invalid
[0m[0m[1m[38;5;10m.LBB12_132:
[0m [0m[1m[38;5;12mmov    [0m byte, ptr, [rbp, -, 80], al
[0m [0m[1m[38;5;12mlea    [0m rax, [rip, +, __unnamed_6]
[0m [0m[1m[38;5;12mmov    [0m qword, ptr, [rsp, +, 32], rax
[0m [0m[1m[38;5;12mlea    [0m rcx, [rip, +, __unnamed_7]
[0m [0m[1m[38;5;12mlea    [0m r9, [rip, +, __unnamed_8]
[0m [0m[1m[38;5;12mlea    [0m r8, [rbp, -, 80]
[0m [0m[1m[38;5;12mmov    [0m edx, 17
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m core::result::unwrap_failed
[0m [0m[1m[38;5;12mjmp    [0m[1m[38;5;10m .LBB12_134
[0m[0m[1m[38;5;10m.LBB12_133:
[0m [0m[1m[38;5;12mcall   [0m[1m[38;5;9m alloc::raw_vec::capacity_overflow
[0m[0m[1m[38;5;10m.LBB12_134:
[0m [0m[1m[38;5;12mud2[0m

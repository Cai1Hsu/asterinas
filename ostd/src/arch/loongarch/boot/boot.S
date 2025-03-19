/* SPDX-License-Identifier: MPL-2.0 */

.macro SET_CSR_BITS CSR_ID, bits_start, bits_end, value
    csrrd      $t0, \CSR_ID

    li.d       $t2, (1 << (\bits_end - \bits_start + 1)) - 1
    slli.d     $t2, $t2, \bits_start

    and        $t0, $t0, $t2

    li.d       $t1, \value
    slli.d     $t1, $t1, \bits_start

    # for safety consideration
    and        $t0, $t0, $t2

    or         $t0, $t0, $t1

    csrwr      $t0, \CSR_ID
.endm

.section .text.entry
.globl _start
_start:
    # 1. Setup DMWIN

    # Configure DMW0. VSEG = 8, PLV0, Strongly ordered uncachd
    li.d        $t0, (0x8000000000000000 | 1)
    csrwr       $t0, 0x180

    # Configure DMW1. VSEG = 9, PLV0, Coherent cached
    li.d        $t0, (0x9000000000000000 | 1 | 1 << 4)
    csrwr       $t0, 0x181

    # 4. Configure MMU

    # PTE width: 0 for 8 bytes
    # 512 entries for each levels of dir/pt
    # | PTE width | PT base | PT width | Dir1_base    | Dir1_width | Dir2_base      | Dir2_width |
    # | 0 << 30   | 12      | 9 << 5   | (12+9) << 10 | 9 << 15    | (12+9+9) << 20 | 9 << 25    |
    li.d        $t0, ((0 << 30) | 12 | (9 << 5) | ((12+9)<<10) | (9<<15) | ((12+9*2)<<20) | (9<<25))
    csrwr       $t0, 0x1c       # LOONGARCH_CSR_PWCL
    # | Dir3_base | dir3_width |
    li.d        $t0, ((12+9*3) | (9<<6))
    csrwr       $t0, 0x1d       # LOONGARCH_CSR_PWCH

    SET_CSR_BITS 0x10, 24, 29, 12 # TLBIDX
    SET_CSR_BITS 0x1e,  0,  5, 12 # STLBPS
    SET_CSR_BITS 0x8e,  0,  5, 12 # TLBREHI

    li.d        $t2, 0x000000ffffffffff # PHYS_ADDR_MASK

    # 2. Setup temporary tlb refill exception handler
    # According to loongarch reference manual, this must be a physical address and 4k aligned
    la.global   $t0, tmp_tlb_refill
    and         $t0, $t0, $t2
    csrwr       $t0, 0x88       # LOONGARCH_CSR_TLBRENTRY

    # 5. Enable MMU
    la.global   $t0, boot_pagetable
    and         $t0, $t0, $t2
    csrwr       $t0, 0x1a       # LOONGARCH_CSR_PGDH
    ori         $t0, $zero, 0x0
    csrwr       $t0, 0x19       # LOONGARCH_CSR_PGDL
    invtlb      0x00, $zero, $zero

    # 6. Enable paging map mode for current mode
    li.w		$t0, 0xb0		# PLV=0, IE=0, PG=1
    csrwr		$t0, 0x0        # LOONGARCH_CSR_CRMD
    li.w		$t0, 0x00		# PLV=0, PIE=0, PWE=0
    csrwr		$t0, 0x1        # LOONGARCH_CSR_PRMD

    # 7. Clear SAVE0 csr, we identify kernel trap by SAVE0
    li.d        $t0, 0x0
    csrwr       $t0, 0x30       # LOONGARCH_CSR_SAVE0

    # li.d        $t1, 0xffffffff00000000 # VIRT_ADDR_OFFSET

    # 8. Set up stack pointer
    la.global   $sp, boot_stack_top
    # or          $sp, $sp, $t1

    # 9. Read cpuid and fdt_base
    csrrd       $a0, 0x20       # cpuid
    li.d        $a1, 0x100000   # FDT_BASE

    # 10. jump to loongarch_boot
    la.global   $t0, loongarch_boot
    # or          $t0, $t0, $t1
    jirl        $zero, $t0, 0

.section .text
.balign 4096
.global tmp_tlb_refill
tmp_tlb_refill:
    csrwr   $t0, 0x8b               # LA_CSR_TLBRSAVE, KScratch for TLB refill exception
    csrrd   $t0, 0x1b               # LA_CSR_PGD, Page table base
    lddir   $t0, $t0, 3
    lddir   $t0, $t0, 2
    lddir   $t0, $t0, 1
    ldpte   $t0, 0
    ldpte   $t0, 1
    tlbfill
    csrrd   $t0, 0x8b               # LA_CSR_TLBRSAVE
    ertn

.section .bss.stack

.globl boot_stack_bottom
boot_stack_bottom:
    .space 0x40000 # 64 KiB

.globl boot_stack_top
boot_stack_top:

.section .data

.align 12
.globl boot_pagetable
boot_pagetable:
    .zero 8 * 256
    .quad 0 # [256] To be assigned with phys(boot_pagetable_2nd)
    .zero 8 * 191
    .quad 0 # [448] To be assigned with phys(boot_pagetable_2nd)
    .zero 8 * 63

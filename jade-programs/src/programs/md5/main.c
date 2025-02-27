void md5_init(void);
void __fastcall__ md5_next_block_fastcall(unsigned char size, unsigned char *data);
void md5_finalize(void);
extern unsigned char md5_hash[16];

unsigned char data[64]= "123456789012345678901234567890123456789012345678901234567890123";
unsigned char ref[16]={0x5e, 0x43, 0xd5, 0x50, 0xcf, 0x52, 0xd2, 0x9f,
                       0x50, 0x60, 0xe8, 0x72, 0xd5, 0xf2, 0x62, 0x8d};
int i;
int blocks;
int match;

int main() {
    md5_init();
    blocks = 256;

    for (i = 0; i < blocks; i++) {
        md5_next_block_fastcall(64, data);
    }

    md5_finalize();

    match = 1;
    for (i = 0; i < 16; i++) {
        if(ref[i] != md5_hash[i]) {
            match = 0;
        }
    }

    if(match) {
        __asm__("lda #$BE");
    } else {
        __asm__("lda #$EB");
    }

    return 0;
}
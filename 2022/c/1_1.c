#include <stdio.h>

int main(void)
{
    printf("Hello222, world!\n");
    char *filename = "data1.txt";
    FILE *file = fopen(filename, "r");
    
    const unsigned MAX_LENGTH = 256;
    char buffer[MAX_LENGTH];

    printf("filename pointer: %p\n", filename);
    /* while (fgets(buffer, MAX_LENGTH, file)) */
    /* { */
    /*     printf("%s", buffer); */
    /* } */
    fclose(file);
}




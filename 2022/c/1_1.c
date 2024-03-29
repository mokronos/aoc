#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void swap(int *a, int *b)
{
    int tmp = *a;
    printf("tmp: %d\n", tmp);
    *a = *b;
    *b = tmp;
}

int main(void)
{
    char *filename = "data1.txt";
    FILE *file = fopen(filename, "r");
    
    const unsigned MAX_LENGTH = 256;
    char buffer[MAX_LENGTH];

    char line[MAX_LENGTH];
    int max_calories = 0;
    int cur_calories = 0;

    while (fgets(line, MAX_LENGTH, file) != NULL)
    {
        if (strcmp(line, "\n") == 0)
        {
            if (cur_calories > max_calories)
            {
                max_calories = cur_calories;
            }
            cur_calories = 0;
            continue;
        }
        else
        {
            int calories = atoi(line);
            cur_calories += calories;
        }
    }
    
    fclose(file);

    printf("Max calories: %d\n", max_calories);

    return 0;
}



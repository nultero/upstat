#include <stdio.h>
#include <stdlib.h>

static const char ENG_NOW[] = "/sys/class/power_supply/BAT0/energy_now";
static const char ENG_FUL[] = "/sys/class/power_supply/BAT0/energy_full";

static const double THRESHOLDS[] = { 88.0, 70.0, 50.0, 24.0, 0.0 };
static const char   UCHARS[]     = { '\x80', '\x81', '\x82', '\x83', '\x84' }; // battery unicode chars
static const double MAX          = 0.999;

int main() {
    char buf[20];
    double enow;
    double eful;
    FILE *f = fopen(ENG_NOW, "r");
    fread(buf, sizeof buf, 1, f);
    fclose(f);
    enow = atof(buf);

    f = fopen(ENG_FUL, "r");
    fread(buf, sizeof buf, 1, f);
    fclose(f);
    eful = atof(buf);

    double left = enow / eful;
    if (left > MAX) {
        printf("\xe2\x9a\xa1"); // lightning bolt unicode
        return 0;
    } 

    left *= 100.0;
    for (int i = 0; i < 5; i++) {
        if (left > THRESHOLDS[i]) {
            printf(
                "%.2f%% \xef\x89%c", 
                left, UCHARS[i]
            );
            break;
        }
    }

    return 0;
}

volatile int* const VGA_BEGIN_PTR = (int*) 0x8000;
volatile int* const VGA_END_PTR = (int*) 0x8960;
volatile int* const BTNS_ADDR = (int*) 0x8A02;
volatile int* const LED_ADDR = (int*) 0x8A04;

int cursor_location = 3;
int next_player = 0;

int* columns = (int*) 0x8AF0;


int main(void)
{
  draw_cursor(cursor_location);
  draw_grid_dots();


  for (;;) {
    *LED_ADDR = next_player;
    wait_for_any_btns_down();
    handle_btn_press();

    wait_for_all_btns_up();
  }
}


void wait_for_all_btns_up() {
  while (*BTNS_ADDR != 0);
}
void wait_for_any_btns_down() {
  while (*BTNS_ADDR == 0);
}

void draw_cursor(int column) {
  VGA_BEGIN_PTR[column] = 0xFFFF;
  VGA_BEGIN_PTR[column+10] = 0x7FFE;
  VGA_BEGIN_PTR[column+20] = 0x3FFC;
  VGA_BEGIN_PTR[column+30] = 0x1FF8;
  VGA_BEGIN_PTR[column+40] = 0x0FF0;
  VGA_BEGIN_PTR[column+50] = 0x07E0;
  VGA_BEGIN_PTR[column+60] = 0x03C0;
  VGA_BEGIN_PTR[column+70] = 0x0180;
}
void clear_cursor(int column) {
  VGA_BEGIN_PTR[column] = 0x0000;
  VGA_BEGIN_PTR[column+10] = 0x0000;
  VGA_BEGIN_PTR[column+20] = 0x0000;
  VGA_BEGIN_PTR[column+30] = 0x0000;
  VGA_BEGIN_PTR[column+40] = 0x0000;
  VGA_BEGIN_PTR[column+50] = 0x0000;
  VGA_BEGIN_PTR[column+60] = 0x0000;
  VGA_BEGIN_PTR[column+70] = 0x0000;
}

void draw_row_dots(volatile int* p) {
  int i;
  for (i = 0; i < 7; i ++) {
    p[i] = 0x8001;
  }
}

void draw_grid_dots() {
  volatile int* ptr = VGA_END_PTR - 10;
  int i;
  for (i = 0; i < 6; i ++) {
    draw_row_dots(ptr);
    ptr -= 150;
    draw_row_dots(ptr);
    ptr -= 10;
  }
}

void draw_player_0_disc(volatile int* p) {
  p[10] = 0x07E0;
  p[20] = 0x0FF0;
  p[30] = 0x1FF8;
  p[40] = 0x3FFC;
  p[50] = 0x7FFE;
  p[60] = 0x7FFE;
  p[70] = 0x7FFE;
  p[80] = 0x7FFE;
  p[90] = 0x7FFE;
  p[100] = 0x7FFE;
  p[110] = 0x3FFC;
  p[120] = 0x1FF8;
  p[130] = 0x0FF0;
  p[140] = 0x07E0;
}

void draw_player_1_disc(volatile int* p) {
  p[10] = 0x0540;
  p[20] = 0x0AA0;
  p[30] = 0x1550;
  p[40] = 0x2AA8;
  p[50] = 0x5554;
  p[60] = 0x2AAA;
  p[70] = 0x5554;
  p[80] = 0x2AAA;
  p[90] = 0x5554;
  p[100] = 0x2AAA;
  p[110] = 0x1554;
  p[120] = 0x0AA8;
  p[130] = 0x0550;
  p[140] = 0x02A0;
}

void draw_disc(int player, int row, int col) {
  volatile int* p = VGA_END_PTR - 10;
  p += row;
  p -= 150;
  int i;
  for (i = 0; i < col; i ++) {
    p -= 160;
  }
  if (player == 0) {
    draw_player_0_disc(p);
  } else {
    draw_player_1_disc(p);
  }
}

void handle_left_btn() {
  clear_cursor(cursor_location);
  cursor_location --;
  if (cursor_location < 0) {
    cursor_location = 6;
  }
  draw_cursor(cursor_location);
}

void handle_right_btn() {
  clear_cursor(cursor_location);
  cursor_location ++;
  if (cursor_location > 6) {
    cursor_location = 0;
  }
  draw_cursor(cursor_location);
}

void handle_middle_btn() {
  int col = columns[cursor_location];
  if ((col & 0x0C00) != 0) {
    return;
  }
  int num_discs_in_col = 0;
  while (col != 0) {
    col = col >> 2;
    num_discs_in_col ++;
  }
  int new_disc;
  if (next_player == 0) {
    new_disc = 0x1;
  } else {
    new_disc = 0x2;
  }
  int i;
  for (i = 0; i < num_discs_in_col; i ++) {
    new_disc = new_disc << 2;
  }

  columns[cursor_location] |= new_disc;
  draw_disc(next_player, cursor_location, num_discs_in_col);
  next_player = !next_player;
}


void handle_btn_press() {
  int x = *BTNS_ADDR;
  if (x == 0x04) {
    handle_left_btn();
  } else if (x == 0x02) {
    handle_right_btn();
  } else if (x == 0x01) {
    handle_middle_btn();
  }
}





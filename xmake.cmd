rem(){ :;};rem '
@goto b
';make -f makefiles/Makefile.unixy $*; exit
:b
nmake /f makefiles\Makefile.windowsy %*

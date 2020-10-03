COMP = kotlinc
EXEC = kotlin
SRC = Main.kt
CLASS = MainKt.class
INF = META-INF
# Main.class -> Main
APP = $(basename $(CLASS) .class)
TESTDIR = Testfiles
# 暗黙ルールを無効にする, ディレクトリ移動時の出力を無効にする
MAKEFLAGS :=+ --no-builtin-rules --no-print-directory
#
# 引数（ターゲット）なし実行でtestをターゲットにする
all: test

build:$(BIN) 
$(CLASS): $(SRC)
	$(COMP) $(SRC)

# make run は手作業で標準入力を行う実行
run: $(CLASS)
	$(EXEC) $(APP)

# make test
# TESTDIR内のtestファイルの内容を標準入力する実行（空のファイルは無視する）
test: $(CLASS)
	@for testfile in $(wildcard $(TESTDIR)/*.test) ; \
	do \
		if [ -s $$testfile ]; \
			then \
			echo ---------------------- ;\
			echo "exec $$testfile" ;\
			echo "input: " ; \
			cat $$testfile ; \
			echo "" ;\
			echo "output: " ;\
			cat $$testfile | $(EXEC) $(APP) ;\
		fi \
	done \

.phony:clean
clean:
	-rm -r $(INF)
	-rm *.class

# testファイルをすべて空にする
.phony:clean_test
clean_test:
	@for testfile in $(wildcard $(TESTDIR)/*.test) ; \
	do \
		: > $$testfile ;\
	done


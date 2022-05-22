package dev.taah.util;

import lombok.Getter;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.ThreadLocalRandom;
import java.util.stream.Collectors;
import java.util.stream.Stream;

/**
 * @author Taah
 * @project crewmate
 * @since 7:34 PM [20-05-2022]
 */

@Getter
public class GameCode
{
    private static final char[] CHAR_SET = "QWXRTYLPESDFGHUJKZOCVBINMA".toCharArray();
    private static final int[] CHAR_MAP = {25, 21, 19, 10, 8, 11, 12, 13, 22, 15, 16, 6, 24, 23, 18, 7, 0, 3, 9, 4, 14, 20, 1, 2, 5, 17};

    private final String gameCode;
    private final int gameId;
    public GameCode(String gameCode) {
        this.gameCode = gameCode;
        this.gameId = codeToInt(gameCode);
    }

    public GameCode(int gameId) {
        this.gameCode = intToCode(gameId);
        this.gameId = gameId;
    }
    public String intToCode(int gameId)
    {
        if (gameId < -1)
        {
            // Version 2 codes will always be negative
            int firstTwo = (int) (gameId & 0x3FF);
            int lastFour = (int) ((gameId >> 10) & 0xFFFFF);

            return Stream.of(
                    CHAR_SET[firstTwo % 26],
                    CHAR_SET[firstTwo / 26],
                    CHAR_SET[lastFour % 26],
                    CHAR_SET[(lastFour /= 26) % 26],
                    CHAR_SET[(lastFour /= 26) % 26],
                    CHAR_SET[lastFour / 26 % 26]
            ).map(String::valueOf).collect(Collectors.joining());
        } else
        {
            // Version 1 codes will always be positive
            return new String(
                    ByteBuffer.allocate(4).order(ByteOrder.LITTLE_ENDIAN).putInt(Math.toIntExact(gameId)).array(),
                    StandardCharsets.UTF_8
            );
        }
    }

    public int codeToInt(String gameCode)
            throws IllegalArgumentException
    {
        gameCode = gameCode.toUpperCase();

        if (gameCode.chars().anyMatch(character -> !Character.isLetter(character)))
        {
            throw new IllegalArgumentException("Invalid code, expected letters only: " + gameCode);
        }

        if (gameCode.length() == 4)
        {
            return ByteBuffer.wrap(gameCode.getBytes()).order(ByteOrder.LITTLE_ENDIAN).getInt();
        }

        if (gameCode.length() != 6)
        {
            throw new IllegalArgumentException("Invalid code length, expected 4 or 6 characters: " + gameCode);
        }

        int first = CHAR_MAP[(int) gameCode.charAt(0) - 65];
        int second = CHAR_MAP[(int) gameCode.charAt(1) - 65];
        int third = CHAR_MAP[(int) gameCode.charAt(2) - 65];
        int fourth = CHAR_MAP[(int) gameCode.charAt(3) - 65];
        int fifth = CHAR_MAP[(int) gameCode.charAt(4) - 65];
        int sixth = CHAR_MAP[(int) gameCode.charAt(5) - 65];

        int firstTwo = (first + 26 * second) & 0x3FF;
        int lastFour = (third + 26 * (fourth + 26 * (fifth + 26 * sixth)));

        return firstTwo | ((lastFour << 10) & 0x3FFFFC00) | 0x80000000;
    }

    public static GameCode generateCode()
    {
        StringBuilder s = new StringBuilder();
        for (int i = 0; i < 6; i++)
        {
            s.append(CHAR_SET[ThreadLocalRandom.current().nextInt(CHAR_SET.length)]);
        };
        return new GameCode(s.toString());
    }

    @Override
    public boolean equals(Object obj)
    {
        if (!(obj instanceof GameCode other)) return false;
        return other.getGameCode().equals(this.getGameCode()) && other.getGameId() == this.getGameId();
    }
}

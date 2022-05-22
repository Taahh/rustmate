package dev.taah.util;

import lombok.Data;

@Data
public class Vector2
{
    private static final FloatRange RANGE_X = new FloatRange(-50.0f, 50.0f);
    private static final FloatRange RANGE_Y = new FloatRange(-50.0f, 50.0f);

    private final float x, y;

    public static Vector2 readVector2(PacketBuffer reader)
    {
        float x = (float) reader.readUInt16() / 65535.0f;
        float y = (float) reader.readUInt16() / 65535.0f;

        return new Vector2(RANGE_X.lerp(x), RANGE_Y.lerp(y));
    }

    public static void writeVector2(PacketBuffer writer, Vector2 vec)
    {
        // These are uint16, but we declare them as an int since Java has no unsigned primitives
        int x = (int) (RANGE_X.reverseLerp(vec.x) * 65535.0f);
        int y = (int) (RANGE_Y.reverseLerp(vec.y) * 65535.0f);

        writer.writeUInt16(x);
        writer.writeUInt16(y);
    }
}

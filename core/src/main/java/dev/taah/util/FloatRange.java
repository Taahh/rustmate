package dev.taah.util;

import lombok.Data;

@Data
public class FloatRange
{
    private final float min, max;

    /*
     * Used when reading a Vector2
     */
    public float lerp(float value)
    {
        if (0.0 > value)
        {
            value = 0.0f;
        } else if (1.0 < value)
        {
            value = 1.0f;
        }

        return this.min + ((this.max - this.min) * value);
    }

    /*
     * Used when writing a Vector2
     */
    public float reverseLerp(float value)
    {
        value = (value - this.min) / (this.max - this.min);

        if (0.0 > value)
        {
            value = 0.0f;
        } else if (1.0 < value)
        {
            value = 1.0f;
        }

        return value;
    }
}

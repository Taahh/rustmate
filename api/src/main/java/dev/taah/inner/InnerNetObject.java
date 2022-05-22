package dev.taah.inner;

import dev.taah.util.HazelMessage;
import lombok.Data;

/**
 * @author Taah
 * @project crewmate
 * @since 8:32 PM [20-05-2022]
 */
@Data
public class InnerNetObject
{
    private final int netId;
    private final HazelMessage data;
}

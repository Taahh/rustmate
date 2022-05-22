package dev.taah.data;

import dev.taah.util.PacketBuffer;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.Setter;

/**
 * @author Taah
 * @project crewmate
 * @since 7:09 PM [20-05-2022]
 */
@Getter
@Setter
@RequiredArgsConstructor
public class GameOptionsData implements IGameOptionsData
{
    private final long length;
    private final byte version;

    private byte maxPlayers;
    private long keywords;
    private byte maps;
    private float playerSpeedMod;
    private float crewLightMod;
    private float imposterLightMod;
    private float killCooldown;
    private byte commonTasks;
    private byte longTasks;
    private byte shortTasks;
    private int emergencyMeetings;
    private byte imposterCount;
    private byte killDistance;
    private int discussionTime;
    private int votingTime;
    private boolean defaults;
    private byte emergencyCooldown;
    private boolean confirmEjects;
    private boolean visualTasks;
    private boolean anonymousVoting;
    private byte taskBarMode;

    public static GameOptionsData deserialize(PacketBuffer buffer)
    {
        GameOptionsData gameOptionsData = new GameOptionsData(buffer.readPackedUInt32(), buffer.readByte());
        gameOptionsData.maxPlayers = buffer.readByte();
        gameOptionsData.keywords = buffer.readUInt32();
        gameOptionsData.maps = buffer.readByte();
        gameOptionsData.playerSpeedMod = buffer.readFloatLE();
        gameOptionsData.crewLightMod = buffer.readFloatLE();
        gameOptionsData.imposterLightMod = buffer.readFloatLE();
        gameOptionsData.killCooldown = buffer.readFloatLE();
        gameOptionsData.commonTasks = buffer.readByte();
        gameOptionsData.longTasks = buffer.readByte();
        gameOptionsData.shortTasks = buffer.readByte();
        gameOptionsData.emergencyMeetings = buffer.readInt32();
        gameOptionsData.imposterCount = buffer.readByte();
        gameOptionsData.killDistance = buffer.readByte();
        gameOptionsData.discussionTime = buffer.readInt32();
        gameOptionsData.votingTime = buffer.readInt32();
        gameOptionsData.defaults = buffer.readBoolean();

        if (gameOptionsData.version > 1) {
            gameOptionsData.emergencyCooldown = buffer.readByte();
        }
        if (gameOptionsData.version > 2)
        {
            gameOptionsData.confirmEjects = buffer.readBoolean();
            gameOptionsData.visualTasks = buffer.readBoolean();
        }

        if (gameOptionsData.version > 3)
        {
            gameOptionsData.anonymousVoting = buffer.readBoolean();
            gameOptionsData.taskBarMode = buffer.readByte();
        }

        //TODO: Do role options

        return gameOptionsData;
    }


}

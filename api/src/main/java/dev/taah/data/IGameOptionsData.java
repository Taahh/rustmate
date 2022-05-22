package dev.taah.data;

/**
 * @author Taah
 * @project crewmate
 * @since 7:03 PM [20-05-2022]
 */
public interface IGameOptionsData
{

    long getLength();

    byte getVersion();

    byte getMaxPlayers();

    long getKeywords();

    byte getMaps();

    float getPlayerSpeedMod();

    float getCrewLightMod();

    float getImposterLightMod();

    float getKillCooldown();

    byte getCommonTasks();

    byte getLongTasks();

    byte getShortTasks();

    int getEmergencyMeetings();

    byte getImposterCount();

    byte getKillDistance();

    int getDiscussionTime();

    int getVotingTime();

    boolean isDefaults();
    
    byte getEmergencyCooldown();

    boolean isConfirmEjects();

    boolean isVisualTasks();

    boolean isAnonymousVoting();

    byte getTaskBarMode();
}
